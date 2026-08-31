pub mod handshake;
pub mod runtime;

pub use handshake::*;
pub use runtime::*;

use crate::error::{ProtocolError, ProtocolResult};
use serde::{Deserialize, Serialize};

/// Discriminated union of all supported message payloads in Protocol v1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessagePayload {
    WorkerHello(WorkerHello),
    HostConfigure(HostConfigure),
    WorkerReady(WorkerReady),
    HostStart(HostStart),
    RunProgress(RunProgress),
    RunWaitingForInput(RunWaitingForInput),
    ProviderRequest(ProviderRequest),
    ToolRequest(ToolRequest),
    ProposalCreated(ProposalCreated),
    RunResult(RunResult),
    RunFailure(RunFailure),
    WorkerHeartbeat(WorkerHeartbeat),
    ProviderResponse(ProviderResponse),
    ToolResponse(ToolResponse),
    InputResponse(InputResponse),
    BudgetUpdated(BudgetUpdated),
    RunCancel(RunCancel),
    WorkerShutdown(WorkerShutdown),
}

impl MessagePayload {
    pub fn message_type_str(&self) -> &'static str {
        match self {
            MessagePayload::WorkerHello(_) => "worker.hello",
            MessagePayload::HostConfigure(_) => "host.configure",
            MessagePayload::WorkerReady(_) => "worker.ready",
            MessagePayload::HostStart(_) => "host.start",
            MessagePayload::RunProgress(_) => "run.progress",
            MessagePayload::RunWaitingForInput(_) => "run.waitingForInput",
            MessagePayload::ProviderRequest(_) => "provider.request",
            MessagePayload::ToolRequest(_) => "tool.request",
            MessagePayload::ProposalCreated(_) => "proposal.created",
            MessagePayload::RunResult(_) => "run.result",
            MessagePayload::RunFailure(_) => "run.failure",
            MessagePayload::WorkerHeartbeat(_) => "worker.heartbeat",
            MessagePayload::ProviderResponse(_) => "provider.response",
            MessagePayload::ToolResponse(_) => "tool.response",
            MessagePayload::InputResponse(_) => "input.response",
            MessagePayload::BudgetUpdated(_) => "budget.updated",
            MessagePayload::RunCancel(_) => "run.cancel",
            MessagePayload::WorkerShutdown(_) => "worker.shutdown",
        }
    }

    pub fn to_json_value(&self) -> ProtocolResult<serde_json::Value> {
        let val = match self {
            MessagePayload::WorkerHello(p) => serde_json::to_value(p)?,
            MessagePayload::HostConfigure(p) => serde_json::to_value(p)?,
            MessagePayload::WorkerReady(p) => serde_json::to_value(p)?,
            MessagePayload::HostStart(p) => serde_json::to_value(p)?,
            MessagePayload::RunProgress(p) => serde_json::to_value(p)?,
            MessagePayload::RunWaitingForInput(p) => serde_json::to_value(p)?,
            MessagePayload::ProviderRequest(p) => serde_json::to_value(p)?,
            MessagePayload::ToolRequest(p) => serde_json::to_value(p)?,
            MessagePayload::ProposalCreated(p) => serde_json::to_value(p)?,
            MessagePayload::RunResult(p) => serde_json::to_value(p)?,
            MessagePayload::RunFailure(p) => serde_json::to_value(p)?,
            MessagePayload::WorkerHeartbeat(p) => serde_json::to_value(p)?,
            MessagePayload::ProviderResponse(p) => serde_json::to_value(p)?,
            MessagePayload::ToolResponse(p) => serde_json::to_value(p)?,
            MessagePayload::InputResponse(p) => serde_json::to_value(p)?,
            MessagePayload::BudgetUpdated(p) => serde_json::to_value(p)?,
            MessagePayload::RunCancel(p) => serde_json::to_value(p)?,
            MessagePayload::WorkerShutdown(p) => serde_json::to_value(p)?,
        };
        Ok(val)
    }

    pub fn from_type_and_value(message_type: &str, val: serde_json::Value) -> ProtocolResult<Self> {
        let payload = match message_type {
            "worker.hello" => MessagePayload::WorkerHello(serde_json::from_value(val)?),
            "host.configure" => MessagePayload::HostConfigure(serde_json::from_value(val)?),
            "worker.ready" => MessagePayload::WorkerReady(serde_json::from_value(val)?),
            "host.start" => MessagePayload::HostStart(serde_json::from_value(val)?),
            "run.progress" => MessagePayload::RunProgress(serde_json::from_value(val)?),
            "run.waitingForInput" => {
                MessagePayload::RunWaitingForInput(serde_json::from_value(val)?)
            }
            "provider.request" => MessagePayload::ProviderRequest(serde_json::from_value(val)?),
            "tool.request" => MessagePayload::ToolRequest(serde_json::from_value(val)?),
            "proposal.created" => MessagePayload::ProposalCreated(serde_json::from_value(val)?),
            "run.result" => MessagePayload::RunResult(serde_json::from_value(val)?),
            "run.failure" => MessagePayload::RunFailure(serde_json::from_value(val)?),
            "worker.heartbeat" => MessagePayload::WorkerHeartbeat(serde_json::from_value(val)?),
            "provider.response" => MessagePayload::ProviderResponse(serde_json::from_value(val)?),
            "tool.response" => MessagePayload::ToolResponse(serde_json::from_value(val)?),
            "input.response" => MessagePayload::InputResponse(serde_json::from_value(val)?),
            "budget.updated" => MessagePayload::BudgetUpdated(serde_json::from_value(val)?),
            "run.cancel" => MessagePayload::RunCancel(serde_json::from_value(val)?),
            "worker.shutdown" => MessagePayload::WorkerShutdown(serde_json::from_value(val)?),
            other => return Err(ProtocolError::UnknownMessageType(other.to_string())),
        };
        Ok(payload)
    }
}
