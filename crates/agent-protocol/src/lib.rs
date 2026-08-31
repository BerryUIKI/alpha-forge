//! Typed IPC protocol contracts and framing for AlphaForge Agent subprocess workers.
//!
//! Governed by ADR-0010 (Managed Agent Worker Subprocess Architecture) and SUBPROCESS_ROADMAP.md (AW1).

pub mod codec;
pub mod envelope;
pub mod error;
pub mod fixture;
pub mod messages;
pub mod validator;

pub use codec::*;
pub use envelope::*;
pub use error::*;
pub use fixture::*;
pub use messages::*;
pub use validator::*;

/// Current canonical protocol version.
pub const PROTOCOL_VERSION: u32 = 1;

/// Default maximum frame size in bytes (1 MiB).
pub const DEFAULT_MAX_FRAME_BYTES: usize = 1024 * 1024;

/// Default maximum aggregate output per run in bytes (16 MiB).
pub const DEFAULT_MAX_AGGREGATE_BYTES: usize = 16 * 1024 * 1024;
