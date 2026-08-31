use crate::error::{ProtocolError, ProtocolResult};
use crate::messages::MessagePayload;
use crate::PROTOCOL_VERSION;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Generic protocol envelope wrapping any serializable payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolEnvelope<T = serde_json::Value> {
    pub protocol_version: u32,
    pub run_id: String,
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub payload: T,
}

/// Unparsed raw envelope where payload is retained as a dynamic JSON Value.
pub type RawEnvelope = ProtocolEnvelope<serde_json::Value>;

impl<T> ProtocolEnvelope<T> {
    /// Creates a new envelope with a freshly generated UUID for message_id and protocol version 1.
    pub fn new(run_id: impl Into<String>, message_type: impl Into<String>, payload: T) -> Self {
        Self {
            protocol_version: PROTOCOL_VERSION,
            run_id: run_id.into(),
            message_id: Uuid::new_v4().to_string(),
            reply_to: None,
            message_type: message_type.into(),
            payload,
        }
    }

    /// Sets the reply_to correlation field.
    pub fn with_reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    /// Validates version and non-empty IDs.
    pub fn validate_header(&self, expected_run_id: Option<&str>) -> ProtocolResult<()> {
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(ProtocolError::VersionMismatch {
                expected: PROTOCOL_VERSION,
                actual: self.protocol_version,
            });
        }
        if self.run_id.trim().is_empty() {
            return Err(ProtocolError::Validation("run_id cannot be empty".into()));
        }
        if self.message_id.trim().is_empty() {
            return Err(ProtocolError::Validation(
                "message_id cannot be empty".into(),
            ));
        }
        if let Some(expected) = expected_run_id {
            if self.run_id != expected {
                return Err(ProtocolError::RunIdMismatch {
                    expected: expected.to_string(),
                    actual: self.run_id.clone(),
                });
            }
        }
        Ok(())
    }
}

impl RawEnvelope {
    /// Deserializes the raw JSON payload into a strongly typed `MessagePayload`.
    pub fn to_typed(&self) -> ProtocolResult<TypedEnvelope> {
        self.validate_header(None)?;
        let payload =
            MessagePayload::from_type_and_value(&self.message_type, self.payload.clone())?;

        Ok(ProtocolEnvelope {
            protocol_version: self.protocol_version,
            run_id: self.run_id.clone(),
            message_id: self.message_id.clone(),
            reply_to: self.reply_to.clone(),
            message_type: self.message_type.clone(),
            payload,
        })
    }
}

pub type TypedEnvelope = ProtocolEnvelope<MessagePayload>;

impl TypedEnvelope {
    /// Creates a typed envelope from a MessagePayload.
    pub fn from_payload(run_id: impl Into<String>, payload: MessagePayload) -> Self {
        let message_type = payload.message_type_str();
        Self::new(run_id, message_type, payload)
    }

    /// Converts into a raw JSON envelope suitable for newline JSON framing.
    pub fn into_raw(self) -> ProtocolResult<RawEnvelope> {
        let payload_json = self.payload.to_json_value()?;

        Ok(ProtocolEnvelope {
            protocol_version: self.protocol_version,
            run_id: self.run_id,
            message_id: self.message_id,
            reply_to: self.reply_to,
            message_type: self.message_type,
            payload: payload_json,
        })
    }
}
