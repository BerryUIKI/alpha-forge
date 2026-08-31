use crate::envelope::TypedEnvelope;
use crate::messages::*;
use uuid::Uuid;

/// Deterministic test fixtures for protocol verification across host and worker backends.
pub struct ProtocolFixture;

impl ProtocolFixture {
    pub const FIXTURE_WORKER_ID: &'static str = "alphaforge-fixture-worker";
    pub const FIXTURE_WORKER_VERSION: &'static str = "0.1.0";
    pub const FIXTURE_NONCE: &'static str = "test-nonce-secure-proof-12345";
    pub const FIXTURE_SCHEMA_ID: &'static str = "research.investment.v1";

    /// Produces a full valid 4-step handshake sequence.
    pub fn handshake_sequence(
        run_id: &str,
    ) -> (TypedEnvelope, TypedEnvelope, TypedEnvelope, TypedEnvelope) {
        let hello = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::WorkerHello(WorkerHello {
                worker_id: Self::FIXTURE_WORKER_ID.into(),
                worker_version: Self::FIXTURE_WORKER_VERSION.into(),
                protocol_versions: vec![1],
                supported_features: vec!["provider.broker".into(), "tool.broker".into()],
            }),
        );

        let configure = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::HostConfigure(HostConfigure {
                selected_version: 1,
                run_scope: RunScope {
                    workspace_id: "ws-fixture-001".into(),
                    task_id: "task-fixture-001".into(),
                    session_id: Some("session-fixture-001".into()),
                },
                limits: RunLimits::default(),
                capabilities: vec!["research.read".into(), "openai.gpt-4o".into()],
                nonce: Self::FIXTURE_NONCE.into(),
            }),
        );

        let ready = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::WorkerReady(WorkerReady {
                nonce_proof: Self::FIXTURE_NONCE.into(),
                supported_features: vec!["provider.broker".into(), "tool.broker".into()],
            }),
        );

        let start = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::HostStart(HostStart {
                task_input: serde_json::json!({
                    "query": "Evaluate NVDA AI chip competitive moat",
                    "sources": ["sec-10k", "earnings-call"]
                }),
                output_schema_id: Self::FIXTURE_SCHEMA_ID.into(),
                output_schema_version: 1,
            }),
        );

        (hello, configure, ready, start)
    }

    /// Produces a valid terminal research result payload.
    pub fn valid_research_result() -> serde_json::Value {
        serde_json::json!({
            "summary": "NVIDIA maintains a high competitive moat driven by CUDA software lock-in and high-bandwidth memory packaging supply advantages.",
            "claims": [
                {
                    "statement": "CUDA platform developer ecosystem creates high switching costs.",
                    "confidence": 0.92,
                    "evidenceSourceIds": ["doc-sec-10k"]
                }
            ],
            "risks": [
                "Hyperscaler in-house ASIC silicon adoption",
                "Advanced packaging capacity bottlenecks"
            ],
            "confidenceScore": 0.88
        })
    }

    /// Produces a full end-to-end trace of a successful research task.
    pub fn successful_run_trace(run_id: &str) -> Vec<TypedEnvelope> {
        let (hello, configure, ready, start) = Self::handshake_sequence(run_id);

        let progress1 = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::RunProgress(RunProgress {
                step: "fetching_sources".into(),
                percentage: Some(0.25),
                message: "Acquiring primary document context".into(),
            }),
        );

        let tool_req_id = Uuid::new_v4().to_string();
        let tool_req = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::ToolRequest(ToolRequest {
                request_id: tool_req_id.clone(),
                tool_name: "research.search_chunks".into(),
                parameters: serde_json::json!({ "query": "CUDA ecosystem" }),
            }),
        );

        let tool_resp = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::ToolResponse(ToolResponse {
                request_id: tool_req_id,
                result: serde_json::json!({
                    "chunks": [
                        { "id": "chk-01", "content": "Over 4 million developers use CUDA worldwide." }
                    ]
                }),
            }),
        );

        let provider_req_id = Uuid::new_v4().to_string();
        let provider_req = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::ProviderRequest(ProviderRequest {
                request_id: provider_req_id.clone(),
                provider: "openai".into(),
                model: "gpt-4o".into(),
                prompt: "Synthesize competitive moat findings".into(),
                max_tokens: Some(2048),
                temperature: Some(0.2),
            }),
        );

        let provider_resp = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::ProviderResponse(ProviderResponse {
                request_id: provider_req_id,
                content: "Synthesized research analysis findings.".into(),
                usage: Some(ProviderUsage {
                    prompt_tokens: Some(1500),
                    completion_tokens: Some(400),
                    total_tokens: Some(1900),
                    reasoning_tokens: Some(0),
                    estimated_cost_usd: Some(0.012),
                }),
            }),
        );

        let result = TypedEnvelope::from_payload(
            run_id,
            MessagePayload::RunResult(RunResult {
                schema_id: Self::FIXTURE_SCHEMA_ID.into(),
                schema_version: 1,
                result: Self::valid_research_result(),
            }),
        );

        vec![
            hello,
            configure,
            ready,
            start,
            progress1,
            tool_req,
            tool_resp,
            provider_req,
            provider_resp,
            result,
        ]
    }
}
