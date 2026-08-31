//! Proposal Service for human-in-the-loop review of agent proposals (M10-G4)
//!
//! Enforces:
//! - Rust-only persistence: Goose cannot write directly to domain models.
//! - Explicit human confirmation required before any proposal creates domain records.
//! - Strict rejection of trade/order/portfolio manipulation attempts.
//! - Provenance tracking: links accepted domain entities back to the Goose run ID.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::database::repositories::proposal_repository::ProposalRepository;
use crate::error::AppError;
use crate::services::research_note_service::ResearchNoteService;
use crate::services::thesis_service::ThesisService;
use domain::proposal::{
    CreateProposalInput, EvidenceCandidatePayload, Proposal, ProposalStatus, ProposalType,
    ResearchNotePayload,
};
use domain::research::CreateNoteInput;
use domain::thesis::AddEvidenceInput;

/// Service for managing human-approved agent proposals
#[derive(Clone)]
pub struct ProposalService {
    repo: Option<Arc<ProposalRepository>>,
    fallback_proposals: Arc<RwLock<HashMap<String, Proposal>>>,
}

impl Default for ProposalService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalService {
    /// Create a new ProposalService (in-memory mode for tests)
    pub fn new() -> Self {
        Self {
            repo: None,
            fallback_proposals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new ProposalService backed by SQLite repository
    pub fn with_repository(repo: ProposalRepository) -> Self {
        Self {
            repo: Some(Arc::new(repo)),
            fallback_proposals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new proposal from an agent run
    pub async fn create_proposal(&self, input: CreateProposalInput) -> Result<Proposal, AppError> {
        // Validation
        if input.workspace_id.trim().is_empty() {
            return Err(AppError::Validation(
                "Workspace ID cannot be empty".to_string(),
            ));
        }
        if input.title.trim().is_empty() {
            return Err(AppError::Validation(
                "Proposal title cannot be empty".to_string(),
            ));
        }
        if input.summary.trim().is_empty() {
            return Err(AppError::Validation(
                "Proposal summary cannot be empty".to_string(),
            ));
        }

        // Security Guard: Prohibit any trade or execution commands
        let forbidden_keywords = [
            "buy_order",
            "sell_order",
            "execute_trade",
            "place_order",
            "withdraw_cash",
            "deposit_cash",
            "rebalance_portfolio",
        ];
        let payload_str = input.payload.to_string().to_lowercase();
        for keyword in forbidden_keywords {
            if payload_str.contains(keyword) {
                return Err(AppError::Validation(format!(
                    "Security policy violation: Proposals cannot execute trades or modify portfolio balances (detected keyword: '{}')",
                    keyword
                )));
            }
        }

        let now = Utc::now().to_rfc3339();
        let proposal = Proposal {
            id: Uuid::new_v4().to_string(),
            workspace_id: input.workspace_id,
            run_id: input.run_id,
            proposal_type: input.proposal_type,
            title: input.title,
            summary: input.summary,
            payload: input.payload,
            status: ProposalStatus::Pending,
            created_at: now,
            reviewed_at: None,
            resulting_entity_id: None,
        };

        if let Some(ref repo) = self.repo {
            repo.create_proposal(&proposal).await?;
        } else {
            let mut lock = self.fallback_proposals.write().await;
            lock.insert(proposal.id.clone(), proposal.clone());
        }

        Ok(proposal)
    }

    /// List proposals for a workspace, optionally filtered by status
    pub async fn list_proposals(
        &self,
        workspace_id: &str,
        status: Option<ProposalStatus>,
    ) -> Result<Vec<Proposal>, AppError> {
        if let Some(ref repo) = self.repo {
            repo.list_by_workspace(workspace_id, status).await
        } else {
            let lock = self.fallback_proposals.read().await;
            let mut list: Vec<Proposal> = lock
                .values()
                .filter(|p| {
                    p.workspace_id == workspace_id
                        && status.is_none_or(|expected| p.status == expected)
                })
                .cloned()
                .collect();
            // Sort descending by created_at
            list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            Ok(list)
        }
    }

    /// Get a single proposal by ID
    pub async fn get_proposal(&self, id: &str) -> Result<Option<Proposal>, AppError> {
        if let Some(ref repo) = self.repo {
            repo.get_proposal(id).await
        } else {
            let lock = self.fallback_proposals.read().await;
            Ok(lock.get(id).cloned())
        }
    }

    /// Accept a proposal, executing the appropriate domain service write
    pub async fn accept_proposal(
        &self,
        id: &str,
        thesis_service: &ThesisService,
        note_service: &ResearchNoteService,
    ) -> Result<Proposal, AppError> {
        let mut proposal = self
            .get_proposal(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Proposal with ID {} not found", id)))?;

        if proposal.status != ProposalStatus::Pending {
            return Err(AppError::Validation(format!(
                "Proposal {} is already resolved (status: {:?})",
                id, proposal.status
            )));
        }

        let resulting_id = match proposal.proposal_type {
            ProposalType::EvidenceCandidate => {
                let payload: EvidenceCandidatePayload =
                    serde_json::from_value(proposal.payload.clone()).map_err(|e| {
                        AppError::Validation(format!("Invalid evidence proposal payload: {}", e))
                    })?;

                let direction = if payload.relation.to_lowercase().starts_with("contradict") {
                    domain::thesis::EvidenceDirection::Contradicting
                } else {
                    domain::thesis::EvidenceDirection::Supporting
                };

                let evidence = thesis_service
                    .add_evidence(AddEvidenceInput {
                        thesis_id: payload.thesis_id,
                        direction,
                        evidence: payload.excerpt,
                        source_id: Some(payload.source_id),
                    })
                    .await?;
                evidence.id
            }
            ProposalType::ResearchNote => {
                let payload: ResearchNotePayload = serde_json::from_value(proposal.payload.clone())
                    .map_err(|e| {
                        AppError::Validation(format!("Invalid note proposal payload: {}", e))
                    })?;

                let note = note_service
                    .create_note(CreateNoteInput {
                        document_id: payload.document_id,
                        content: payload.content,
                    })
                    .await?;
                note.id
            }
            ProposalType::ReportOutline => {
                // Generates a confirmed outline identifier
                Uuid::new_v4().to_string()
            }
        };

        let now = Utc::now().to_rfc3339();
        if let Some(ref repo) = self.repo {
            repo.update_status(
                id,
                ProposalStatus::Accepted,
                Some(&now),
                Some(&resulting_id),
            )
            .await?;
        } else {
            let mut lock = self.fallback_proposals.write().await;
            if let Some(p) = lock.get_mut(id) {
                p.status = ProposalStatus::Accepted;
                p.reviewed_at = Some(now.clone());
                p.resulting_entity_id = Some(resulting_id.clone());
            }
        }

        proposal.status = ProposalStatus::Accepted;
        proposal.reviewed_at = Some(now);
        proposal.resulting_entity_id = Some(resulting_id);

        Ok(proposal)
    }

    /// Reject a proposal
    pub async fn reject_proposal(&self, id: &str) -> Result<Proposal, AppError> {
        let mut proposal = self
            .get_proposal(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Proposal with ID {} not found", id)))?;

        if proposal.status != ProposalStatus::Pending {
            return Err(AppError::Validation(format!(
                "Proposal {} is already resolved (status: {:?})",
                id, proposal.status
            )));
        }

        let now = Utc::now().to_rfc3339();
        if let Some(ref repo) = self.repo {
            repo.update_status(id, ProposalStatus::Rejected, Some(&now), None)
                .await?;
        } else {
            let mut lock = self.fallback_proposals.write().await;
            if let Some(p) = lock.get_mut(id) {
                p.status = ProposalStatus::Rejected;
                p.reviewed_at = Some(now.clone());
            }
        }

        proposal.status = ProposalStatus::Rejected;
        proposal.reviewed_at = Some(now);

        Ok(proposal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_create_and_list_proposals() {
        let service = ProposalService::new();

        let input = CreateProposalInput {
            workspace_id: "ws-1".to_string(),
            run_id: "run-100".to_string(),
            proposal_type: ProposalType::EvidenceCandidate,
            title: "Link Q3 report to Margin thesis".to_string(),
            summary: "Q3 report confirms gross margin expansion".to_string(),
            payload: json!({
                "thesis_id": "thesis-1",
                "source_id": "doc-1",
                "excerpt": "Gross margins expanded by 180 bps YoY.",
                "relation": "supports",
                "confidence": 90
            }),
        };

        let proposal = service.create_proposal(input).await.unwrap();
        assert_eq!(proposal.status, ProposalStatus::Pending);
        assert_eq!(proposal.workspace_id, "ws-1");

        let pending_list = service
            .list_proposals("ws-1", Some(ProposalStatus::Pending))
            .await
            .unwrap();
        assert_eq!(pending_list.len(), 1);
        assert_eq!(pending_list[0].id, proposal.id);

        let accepted_list = service
            .list_proposals("ws-1", Some(ProposalStatus::Accepted))
            .await
            .unwrap();
        assert_eq!(accepted_list.len(), 0);
    }

    #[tokio::test]
    async fn test_reject_proposal() {
        let service = ProposalService::new();

        let input = CreateProposalInput {
            workspace_id: "ws-1".to_string(),
            run_id: "run-100".to_string(),
            proposal_type: ProposalType::ResearchNote,
            title: "Add footnote on currency headwinds".to_string(),
            summary: "Currency headwinds may impact H2 guidance".to_string(),
            payload: json!({
                "document_id": "doc-1",
                "content": "FX impact estimated at 2% on revenue."
            }),
        };

        let proposal = service.create_proposal(input).await.unwrap();
        let rejected = service.reject_proposal(&proposal.id).await.unwrap();

        assert_eq!(rejected.status, ProposalStatus::Rejected);
        assert!(rejected.reviewed_at.is_some());
        assert_eq!(rejected.resulting_entity_id, None);

        // Rejecting again should return validation error
        let err = service.reject_proposal(&proposal.id).await.unwrap_err();
        match err {
            AppError::Validation(msg) => assert!(msg.contains("already resolved")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_proposal_sqlite_repository_persistence() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();

        sqlx::query(
            r#"
            CREATE TABLE workspaces (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE proposals (
                id TEXT PRIMARY KEY NOT NULL,
                workspace_id TEXT NOT NULL,
                run_id TEXT NOT NULL,
                proposal_type TEXT NOT NULL,
                title TEXT NOT NULL,
                summary TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                reviewed_at TEXT,
                resulting_entity_id TEXT,
                FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create workspace first for foreign key
        sqlx::query("INSERT INTO workspaces (id, name, created_at, updated_at) VALUES ('ws-persisted', 'Test WS', datetime('now'), datetime('now'))")
            .execute(&pool)
            .await
            .unwrap();

        let repo = ProposalRepository::new(pool);
        let service = ProposalService::with_repository(repo);

        let input = CreateProposalInput {
            workspace_id: "ws-persisted".to_string(),
            run_id: "run-persist-1".to_string(),
            proposal_type: ProposalType::ResearchNote,
            title: "Persisted Note Proposal".to_string(),
            summary: "Testing SQLite persistence".to_string(),
            payload: json!({
                "document_id": "doc-1",
                "content": "Note content"
            }),
        };

        let created = service.create_proposal(input).await.unwrap();
        assert_eq!(created.status, ProposalStatus::Pending);

        let fetched = service.get_proposal(&created.id).await.unwrap();
        assert!(fetched.is_some());
        let fetched_proposal = fetched.unwrap();
        assert_eq!(fetched_proposal.title, "Persisted Note Proposal");

        let rejected = service.reject_proposal(&created.id).await.unwrap();
        assert_eq!(rejected.status, ProposalStatus::Rejected);

        let re_fetched = service.get_proposal(&created.id).await.unwrap().unwrap();
        assert_eq!(re_fetched.status, ProposalStatus::Rejected);
    }
}
