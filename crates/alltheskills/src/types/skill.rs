use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::SkillSource;

/// Represents an AI skill with metadata
///
/// A `Skill` contains all the information needed to identify, describe,
/// and locate a skill from any supported source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier for the skill
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what the skill does
    pub description: String,
    /// Version string (e.g., "1.0.0")
    pub version: Option<String>,
    /// Source location information
    pub source: SkillSource,
    /// Type of source (Claude, GitHub, etc.)
    pub source_type: SourceType,
    /// Local filesystem path to the skill
    pub path: PathBuf,
    /// When the skill was installed
    pub installed_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: SkillMetadata,
    /// Format of the skill
    pub format: SkillFormat,
}

/// Types of skill sources supported by the library
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    /// Claude Code skills
    Claude,
    /// Cline skills
    Cline,
    /// OpenClaw skills
    OpenClaw,
    /// Roo Code (formerly Roo Cline) skills
    RooCode,
    /// OpenAI Codex skills
    OpenAICodex,
    /// Kilo Code skills
    KiloCode,
    /// Moltbot (formerly ClawdBot) skills
    Moltbot,
    /// GitHub repository
    GitHub,
    /// Local filesystem
    Local,
    /// Custom source type
    Custom(String),
}

/// Format of the skill definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillFormat {
    /// Native Claude skill format (claude.json)
    ClaudeSkill,
    /// Claude plugin format
    ClaudePlugin,
    /// Cline skill format (cline.json)
    ClineSkill,
    /// OpenClaw skill format (skill.json)
    OpenClawSkill,
    /// Roo Code skill format (roo.json, .roomodes)
    RooSkill,
    /// OpenAI Codex skill format
    CodexSkill,
    /// Kilo Code skill format
    KiloSkill,
    /// Moltbot skill format (manifest.json, SKILL.md)
    MoltbotSkill,
    /// Generic Markdown documentation
    GenericMarkdown,
    /// Generic JSON configuration
    GenericJson,
    /// Unknown or unsupported format
    Unknown,
}

/// Additional metadata about a skill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    /// Author of the skill
    pub author: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// License identifier
    pub license: Option<String>,
    /// Path to readme file
    pub readme: Option<String>,
    /// Required dependencies or tools
    pub requirements: Vec<String>,
}
