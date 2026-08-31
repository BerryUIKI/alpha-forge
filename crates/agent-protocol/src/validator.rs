use crate::envelope::RawEnvelope;
use crate::error::{ProtocolError, ProtocolResult};
use std::collections::{HashMap, HashSet, VecDeque};

const MAX_SEEN_MESSAGE_IDS: usize = 2048;

/// Handshake state machine transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    Uninitialized,
    ReceivedHello,
    SentConfigure,
    ReceivedReady,
    SentStart,
    Running,
    Terminal,
}

/// Host-side session validator that tracks handshake, request correlation, and message ID uniqueness.
pub struct SessionValidator {
    run_id: String,
    handshake_state: HandshakeState,
    expected_nonce: Option<String>,
    seen_message_ids: HashSet<String>,
    message_id_queue: VecDeque<String>,
    pending_broker_requests: HashMap<String, String>, // request_id -> message_type
    pending_user_inputs: HashSet<String>,
}

impl SessionValidator {
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            handshake_state: HandshakeState::Uninitialized,
            expected_nonce: None,
            seen_message_ids: HashSet::new(),
            message_id_queue: VecDeque::new(),
            pending_broker_requests: HashMap::new(),
            pending_user_inputs: HashSet::new(),
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn handshake_state(&self) -> HandshakeState {
        self.handshake_state
    }

    pub fn set_expected_nonce(&mut self, nonce: impl Into<String>) {
        self.expected_nonce = Some(nonce.into());
    }

    fn track_message_id(&mut self, message_id: String) -> bool {
        if !self.seen_message_ids.insert(message_id.clone()) {
            return false;
        }
        self.message_id_queue.push_back(message_id);
        if self.message_id_queue.len() > MAX_SEEN_MESSAGE_IDS {
            if let Some(oldest) = self.message_id_queue.pop_front() {
                self.seen_message_ids.remove(&oldest);
            }
        }
        true
    }

    /// Validates an incoming message envelope from the worker.
    pub fn validate_incoming(&mut self, envelope: &RawEnvelope) -> ProtocolResult<()> {
        // 1. Header and Run ID checks (worker.hello is allowed before host configures run_id)
        let expected_run = if envelope.message_type == "worker.hello" {
            None
        } else {
            Some(self.run_id.as_str())
        };
        envelope.validate_header(expected_run)?;

        // 2. Duplicate Message ID check (bounded history)
        if !self.track_message_id(envelope.message_id.clone()) {
            return Err(ProtocolError::DuplicateMessageId(
                envelope.message_id.clone(),
            ));
        }

        // 3. Handshake and State checks
        match envelope.message_type.as_str() {
            "worker.hello" => {
                if self.handshake_state != HandshakeState::Uninitialized {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "Uninitialized",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                self.handshake_state = HandshakeState::ReceivedHello;
            }
            "worker.ready" => {
                if self.handshake_state != HandshakeState::SentConfigure {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "SentConfigure",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                // Check nonce proof if present
                if let Some(ref expected) = self.expected_nonce {
                    if let Some(proof) = envelope.payload.get("nonceProof").and_then(|v| v.as_str())
                    {
                        if proof != expected {
                            return Err(ProtocolError::NonceMismatch);
                        }
                    } else {
                        return Err(ProtocolError::MissingField {
                            message_type: "worker.ready",
                            field: "nonceProof",
                        });
                    }
                }
                self.handshake_state = HandshakeState::ReceivedReady;
            }
            "run.progress" | "worker.heartbeat" => {
                if self.handshake_state != HandshakeState::Running
                    && self.handshake_state != HandshakeState::SentStart
                {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "Running or SentStart",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                self.handshake_state = HandshakeState::Running;
            }
            "run.waitingForInput" => {
                if self.handshake_state != HandshakeState::Running
                    && self.handshake_state != HandshakeState::SentStart
                {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "Running",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                self.handshake_state = HandshakeState::Running;
                self.pending_user_inputs.insert(envelope.message_id.clone());
            }
            "provider.request" | "tool.request" => {
                if self.handshake_state != HandshakeState::Running
                    && self.handshake_state != HandshakeState::SentStart
                {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "Running",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                self.handshake_state = HandshakeState::Running;

                if let Some(req_id) = envelope.payload.get("requestId").and_then(|v| v.as_str()) {
                    self.pending_broker_requests
                        .insert(req_id.to_string(), envelope.message_type.clone());
                } else {
                    return Err(ProtocolError::MissingField {
                        message_type: "broker.request",
                        field: "requestId",
                    });
                }
            }
            "proposal.created" => {
                if self.handshake_state != HandshakeState::Running {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "Running",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
            }
            "run.result" | "run.failure" => {
                self.handshake_state = HandshakeState::Terminal;
            }
            unknown => {
                return Err(ProtocolError::UnknownMessageType(unknown.to_string()));
            }
        }

        Ok(())
    }

    /// Records an outgoing host message and advances the state machine.
    pub fn record_outgoing(&mut self, envelope: &RawEnvelope) -> ProtocolResult<()> {
        envelope.validate_header(Some(&self.run_id))?;
        self.track_message_id(envelope.message_id.clone());

        match envelope.message_type.as_str() {
            "host.configure" => {
                if self.handshake_state != HandshakeState::ReceivedHello {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "ReceivedHello",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                if let Some(nonce) = envelope.payload.get("nonce").and_then(|v| v.as_str()) {
                    self.expected_nonce = Some(nonce.to_string());
                }
                self.handshake_state = HandshakeState::SentConfigure;
            }
            "host.start" => {
                if self.handshake_state != HandshakeState::ReceivedReady {
                    return Err(ProtocolError::InvalidHandshakeState {
                        expected: "ReceivedReady",
                        actual: format!("{:?}", self.handshake_state),
                    });
                }
                self.handshake_state = HandshakeState::SentStart;
            }
            "provider.response" | "tool.response" => {
                if let Some(req_id) = envelope.payload.get("requestId").and_then(|v| v.as_str()) {
                    match self.pending_broker_requests.remove(req_id) {
                        Some(expected_req_type) => {
                            let expected_resp_type = match expected_req_type.as_str() {
                                "provider.request" => "provider.response",
                                "tool.request" => "tool.response",
                                _ => "",
                            };
                            if envelope.message_type != expected_resp_type {
                                return Err(ProtocolError::InvalidReplyTo {
                                    message_id: envelope.message_id.clone(),
                                    reply_to: format!(
                                        "{} (expected {}, got {})",
                                        req_id, expected_resp_type, envelope.message_type
                                    ),
                                });
                            }
                        }
                        None => {
                            return Err(ProtocolError::InvalidReplyTo {
                                message_id: envelope.message_id.clone(),
                                reply_to: req_id.to_string(),
                            });
                        }
                    }
                }
            }
            "input.response" => {
                // Clear any pending user input request if present
                self.pending_user_inputs.clear();
            }
            "run.cancel" | "worker.shutdown" => {
                self.handshake_state = HandshakeState::Terminal;
            }
            _ => {}
        }

        Ok(())
    }
}
