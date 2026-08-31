use crate::envelope::RawEnvelope;
use crate::error::{ProtocolError, ProtocolResult};
use crate::{DEFAULT_MAX_AGGREGATE_BYTES, DEFAULT_MAX_FRAME_BYTES};
use std::io::{BufRead, Write};

/// Synchronous frame reader with configurable per-frame and aggregate byte limits.
pub struct SyncFrameReader<R> {
    reader: R,
    max_frame_bytes: usize,
    max_aggregate_bytes: usize,
    total_bytes_read: usize,
}

impl<R: BufRead> SyncFrameReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
            max_aggregate_bytes: DEFAULT_MAX_AGGREGATE_BYTES,
            total_bytes_read: 0,
        }
    }

    pub fn with_limits(reader: R, max_frame_bytes: usize, max_aggregate_bytes: usize) -> Self {
        Self {
            reader,
            max_frame_bytes,
            max_aggregate_bytes,
            total_bytes_read: 0,
        }
    }

    pub fn total_bytes_read(&self) -> usize {
        self.total_bytes_read
    }

    /// Reads the next newline-delimited protocol frame and parses it into a RawEnvelope.
    /// Returns Ok(None) on clean EOF.
    pub fn read_frame(&mut self) -> ProtocolResult<Option<RawEnvelope>> {
        let mut line = String::new();

        // Read line with bounded frame check
        let mut bytes_in_line = 0;
        loop {
            let available = self.reader.fill_buf()?;
            if available.is_empty() {
                if bytes_in_line == 0 {
                    return Ok(None);
                }
                break;
            }

            let newline_pos = available.iter().position(|&b| b == b'\n');
            let take_len = match newline_pos {
                Some(pos) => pos + 1,
                None => available.len(),
            };

            if bytes_in_line + take_len > self.max_frame_bytes {
                return Err(ProtocolError::OversizedFrame {
                    actual_bytes: bytes_in_line + take_len,
                    max_bytes: self.max_frame_bytes,
                });
            }

            if self.total_bytes_read + take_len > self.max_aggregate_bytes {
                return Err(ProtocolError::AggregateOutputLimitExceeded {
                    total_bytes: self.total_bytes_read + take_len,
                    limit_bytes: self.max_aggregate_bytes,
                });
            }

            let s = std::str::from_utf8(&available[..take_len])
                .map_err(|e| ProtocolError::Validation(format!("Invalid UTF-8 in frame: {}", e)))?;
            line.push_str(s);
            bytes_in_line += take_len;
            self.total_bytes_read += take_len;
            self.reader.consume(take_len);

            if newline_pos.is_some() {
                break;
            }
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let envelope: RawEnvelope = serde_json::from_str(trimmed)?;
        Ok(Some(envelope))
    }
}

/// Synchronous frame writer that encodes envelopes as newline-delimited JSON.
pub struct SyncFrameWriter<W> {
    writer: W,
    max_frame_bytes: usize,
    total_bytes_written: usize,
}

impl<W: Write> SyncFrameWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
            total_bytes_written: 0,
        }
    }

    pub fn with_max_frame_bytes(writer: W, max_frame_bytes: usize) -> Self {
        Self {
            writer,
            max_frame_bytes,
            total_bytes_written: 0,
        }
    }

    pub fn total_bytes_written(&self) -> usize {
        self.total_bytes_written
    }

    /// Writes an envelope followed by a newline and flushes the underlying writer.
    pub fn write_frame<T: serde::Serialize>(
        &mut self,
        envelope: &crate::envelope::ProtocolEnvelope<T>,
    ) -> ProtocolResult<()> {
        let serialized = serde_json::to_string(envelope)?;
        let frame_bytes = serialized.as_bytes();

        if frame_bytes.len() > self.max_frame_bytes {
            return Err(ProtocolError::OversizedFrame {
                actual_bytes: frame_bytes.len(),
                max_bytes: self.max_frame_bytes,
            });
        }

        self.writer.write_all(frame_bytes)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;

        self.total_bytes_written += frame_bytes.len() + 1;
        Ok(())
    }
}

#[cfg(feature = "tokio")]
pub mod async_codec {
    use super::*;
    use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

    /// Async frame reader for Tokio streams.
    pub struct AsyncFrameReader<R> {
        reader: R,
        max_frame_bytes: usize,
        max_aggregate_bytes: usize,
        total_bytes_read: usize,
    }

    impl<R: AsyncBufRead + Unpin> AsyncFrameReader<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
                max_aggregate_bytes: DEFAULT_MAX_AGGREGATE_BYTES,
                total_bytes_read: 0,
            }
        }

        pub fn with_limits(reader: R, max_frame_bytes: usize, max_aggregate_bytes: usize) -> Self {
            Self {
                reader,
                max_frame_bytes,
                max_aggregate_bytes,
                total_bytes_read: 0,
            }
        }

        pub fn total_bytes_read(&self) -> usize {
            self.total_bytes_read
        }

        pub async fn read_frame(&mut self) -> ProtocolResult<Option<RawEnvelope>> {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                return Ok(None);
            }

            if bytes_read > self.max_frame_bytes {
                return Err(ProtocolError::OversizedFrame {
                    actual_bytes: bytes_read,
                    max_bytes: self.max_frame_bytes,
                });
            }

            self.total_bytes_read += bytes_read;
            if self.total_bytes_read > self.max_aggregate_bytes {
                return Err(ProtocolError::AggregateOutputLimitExceeded {
                    total_bytes: self.total_bytes_read,
                    limit_bytes: self.max_aggregate_bytes,
                });
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }

            let envelope: RawEnvelope = serde_json::from_str(trimmed)?;
            Ok(Some(envelope))
        }
    }

    /// Async frame writer for Tokio streams.
    pub struct AsyncFrameWriter<W> {
        writer: W,
        max_frame_bytes: usize,
        total_bytes_written: usize,
    }

    impl<W: AsyncWrite + Unpin> AsyncFrameWriter<W> {
        pub fn new(writer: W) -> Self {
            Self {
                writer,
                max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
                total_bytes_written: 0,
            }
        }

        pub fn with_max_frame_bytes(writer: W, max_frame_bytes: usize) -> Self {
            Self {
                writer,
                max_frame_bytes,
                total_bytes_written: 0,
            }
        }

        pub fn total_bytes_written(&self) -> usize {
            self.total_bytes_written
        }

        pub async fn write_frame<T: serde::Serialize>(
            &mut self,
            envelope: &crate::envelope::ProtocolEnvelope<T>,
        ) -> ProtocolResult<()> {
            let serialized = serde_json::to_string(envelope)?;
            let frame_bytes = serialized.as_bytes();

            if frame_bytes.len() > self.max_frame_bytes {
                return Err(ProtocolError::OversizedFrame {
                    actual_bytes: frame_bytes.len(),
                    max_bytes: self.max_frame_bytes,
                });
            }

            self.writer.write_all(frame_bytes).await?;
            self.writer.write_all(b"\n").await?;
            self.writer.flush().await?;

            self.total_bytes_written += frame_bytes.len() + 1;
            Ok(())
        }
    }
}
