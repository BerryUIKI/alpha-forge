use agent_protocol::codec::{SyncFrameReader, SyncFrameWriter};
use agent_protocol::envelope::TypedEnvelope;
use agent_protocol::error::ProtocolResult;
use agent_protocol::fixture::ProtocolFixture;
use agent_protocol::messages::*;
use clap::ValueEnum;
use std::io::{stdin, stdout, Write};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FixtureMode {
    Success,
    Fail,
    Hang,
    Malformed,
    Oversized,
    Cancel,
}

pub struct FixtureWorker {
    mode: FixtureMode,
}

impl FixtureWorker {
    pub fn new(mode: FixtureMode) -> Self {
        Self { mode }
    }

    pub fn run(&self) -> ProtocolResult<()> {
        let mut reader = SyncFrameReader::new(stdin().lock());
        let mut writer = SyncFrameWriter::new(stdout().lock());

        let worker_run_id = Uuid::new_v4().to_string();

        // 1. Send worker.hello
        let hello = TypedEnvelope::from_payload(
            &worker_run_id,
            MessagePayload::WorkerHello(WorkerHello {
                worker_id: ProtocolFixture::FIXTURE_WORKER_ID.into(),
                worker_version: ProtocolFixture::FIXTURE_WORKER_VERSION.into(),
                protocol_versions: vec![1],
                supported_features: vec!["provider.broker".into(), "tool.broker".into()],
            }),
        );
        writer.write_frame(&hello.into_raw()?)?;

        // 2. Read host.configure
        let raw_configure = match reader.read_frame()? {
            Some(frame) => frame,
            None => return Ok(()),
        };
        let typed_configure = raw_configure.to_typed()?;
        let configure = match typed_configure.payload {
            MessagePayload::HostConfigure(c) => c,
            _ => return Ok(()),
        };

        let run_id = typed_configure.run_id;

        // Check for Malformed / Oversized injection points before ready
        if self.mode == FixtureMode::Malformed {
            let mut out = stdout().lock();
            out.write_all(b"{\"protocolVersion\": 1, \"corrupt_unclosed_json\n")?;
            out.flush()?;
            return Ok(());
        }

        if self.mode == FixtureMode::Oversized {
            let huge = "A".repeat(1024 * 1024 + 1024);
            let mut out = stdout().lock();
            let json = serde_json::json!({
                "protocolVersion": 1,
                "runId": run_id,
                "messageId": Uuid::new_v4().to_string(),
                "type": "run.progress",
                "payload": { "message": huge }
            });
            out.write_all(serde_json::to_string(&json)?.as_bytes())?;
            out.write_all(b"\n")?;
            out.flush()?;
            return Ok(());
        }

        // 3. Send worker.ready with nonceProof
        let ready = TypedEnvelope::from_payload(
            &run_id,
            MessagePayload::WorkerReady(WorkerReady {
                nonce_proof: configure.nonce,
                supported_features: vec!["provider.broker".into(), "tool.broker".into()],
            }),
        );
        writer.write_frame(&ready.into_raw()?)?;

        // 4. Read host.start
        let raw_start = match reader.read_frame()? {
            Some(frame) => frame,
            None => return Ok(()),
        };
        let _typed_start = raw_start.to_typed()?;

        // Dispatch based on fixture mode
        match self.mode {
            FixtureMode::Success => {
                // Progress
                let prog = TypedEnvelope::from_payload(
                    &run_id,
                    MessagePayload::RunProgress(RunProgress {
                        step: "researching".into(),
                        percentage: Some(0.5),
                        message: "Analyzing competitive moat".into(),
                    }),
                );
                writer.write_frame(&prog.into_raw()?)?;

                // Result
                let res = TypedEnvelope::from_payload(
                    &run_id,
                    MessagePayload::RunResult(RunResult {
                        schema_id: ProtocolFixture::FIXTURE_SCHEMA_ID.into(),
                        schema_version: 1,
                        result: ProtocolFixture::valid_research_result(),
                    }),
                );
                writer.write_frame(&res.into_raw()?)?;
            }
            FixtureMode::Fail => {
                let fail = TypedEnvelope::from_payload(
                    &run_id,
                    MessagePayload::RunFailure(RunFailure {
                        code: "AGENT_FIXTURE_FAILED".into(),
                        message: "Simulated deterministic failure in fixture worker".into(),
                        recoverable: false,
                    }),
                );
                writer.write_frame(&fail.into_raw()?)?;
                std::process::exit(1);
            }
            FixtureMode::Hang => {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
            FixtureMode::Cancel => {
                // Stream initial progress
                let prog = TypedEnvelope::from_payload(
                    &run_id,
                    MessagePayload::RunProgress(RunProgress {
                        step: "waiting_cancel".into(),
                        percentage: Some(0.1),
                        message: "Working until cancelled".into(),
                    }),
                );
                writer.write_frame(&prog.into_raw()?)?;

                // Wait for cancel frame
                while let Some(raw) = reader.read_frame()? {
                    if let Ok(typed) = raw.to_typed() {
                        if matches!(typed.payload, MessagePayload::RunCancel(_)) {
                            eprintln!("[worker] Received run.cancel, exiting cleanly");
                            break;
                        }
                    }
                }
            }
            FixtureMode::Malformed | FixtureMode::Oversized => unreachable!(),
        }

        Ok(())
    }
}
