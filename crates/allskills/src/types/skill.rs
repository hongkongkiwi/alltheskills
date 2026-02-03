use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::SkillSource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Option<String>,
    pub source: SkillSource,
    pub source_type: SourceType,
    pub path: PathBuf,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub metadata: SkillMetadata,
    pub format: SkillFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Claude,
    Cline,
    OpenClaw,
    RooCode,
    OpenAICodex,
    KiloCode,
    GitHub,
    Local,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillFormat {
    ClaudeSkill,
    ClaudePlugin,
    ClineSkill,
    OpenClawSkill,
    RooSkill,
    CodexSkill,
    GenericMarkdown,
    GenericJson,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub readme: Option<String>,
    pub requirements: Vec<String>,
}
