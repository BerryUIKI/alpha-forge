use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeEntityType {
    Company,
    Industry,
    Technology,
    MacroTheme,
}

impl std::fmt::Display for KnowledgeEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Company => write!(f, "company"),
            Self::Industry => write!(f, "industry"),
            Self::Technology => write!(f, "technology"),
            Self::MacroTheme => write!(f, "macro_theme"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntity {
    pub id: String,
    pub workspace_id: String,
    pub entity_type: KnowledgeEntityType,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelationship {
    pub id: String,
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisEntityLink {
    pub thesis_id: String,
    pub entity_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKnowledgeEntityInput {
    pub workspace_id: String,
    pub entity_type: KnowledgeEntityType,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKnowledgeRelationshipInput {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub relationship_type: String,
}
