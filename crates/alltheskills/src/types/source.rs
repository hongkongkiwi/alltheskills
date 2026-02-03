//! Source types for AllTheSkills
//!
//! This module defines types for representing skill sources, including
//! local filesystem paths, GitHub repositories, and remote URLs.
//!
//! # Source Types
//!
//! - [`SkillSource`] - Location of a skill (local, GitHub, remote)
//! - [`SourceConfig`] - Configuration for a skill source
//! - [`SkillScope`] - Installation scope (global, user, project)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::SourceType;

/// Source location of a skill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    /// Local filesystem path
    Local {
        /// Path to the skill directory
        path: PathBuf,
    },
    /// GitHub repository
    GitHub {
        /// Repository owner
        owner: String,
        /// Repository name
        repo: String,
        /// Subdirectory within the repository (optional)
        subdir: Option<String>,
        /// Git branch (optional)
        branch: Option<String>,
    },
    /// Generic remote URL
    Remote {
        /// URL to fetch from
        url: String,
        /// HTTP headers to include in requests
        headers: Vec<(String, String)>,
    },
}

/// Configuration for a skill source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceConfig {
    /// Display name for this source
    pub name: String,
    /// Type of source
    pub source_type: SourceType,
    /// Whether this source is enabled
    pub enabled: bool,
    /// Scope of the source
    pub scope: SkillScope,
    /// Priority for ordering (higher = earlier)
    pub priority: i32,
}

/// Scope of a skill installation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillScope {
    /// System-wide installation
    Global,
    /// User-specific installation
    User,
    /// Project-specific installation
    Project,
}
