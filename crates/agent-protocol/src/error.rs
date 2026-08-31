use thiserror::Error;

/// Protocol errors encountered during worker framing, deserialization, or validation.
#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u32, actual: u32 },

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u32),

    #[error("Frame exceeds maximum allowed size: {actual_bytes} > {max_bytes} bytes")]
    OversizedFrame {
        actual_bytes: usize,
        max_bytes: usize,
    },

    #[error("Aggregate output exceeds budget: {total_bytes} > {limit_bytes} bytes")]
    AggregateOutputLimitExceeded {
        total_bytes: usize,
        limit_bytes: usize,
    },

    #[error("Invalid JSON payload: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Missing required field '{field}' in message '{message_type}'")]
    MissingField {
        message_type: &'static str,
        field: &'static str,
    },

    #[error("Unknown message type: '{0}'")]
    UnknownMessageType(String),

    #[error("Run ID mismatch: expected {expected}, got {actual}")]
    RunIdMismatch { expected: String, actual: String },

    #[error("Duplicate message ID encountered: {0}")]
    DuplicateMessageId(String),

    #[error("Invalid reply_to: message '{message_id}' replied to unknown request '{reply_to}'")]
    InvalidReplyTo {
        message_id: String,
        reply_to: String,
    },

    #[error("Invalid handshake state: expected {expected}, got {actual}")]
    InvalidHandshakeState {
        expected: &'static str,
        actual: String,
    },

    #[error("Handshake nonce mismatch")]
    NonceMismatch,

    #[error("I/O error during protocol transport: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unexpected EOF on protocol stream")]
    UnexpectedEof,

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type ProtocolResult<T> = Result<T, ProtocolError>;
