use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{SkillScope, SourceConfig};

/// Global configuration for AllTheSkills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSkillsConfig {
    /// Configuration format version
    pub version: u8,
    /// Default scope for new installations
    pub default_scope: SkillScope,
    /// Configured skill sources
    pub sources: Vec<SourceConfig>,
    /// Default installation directory
    pub install_dir: PathBuf,
    /// Cache directory for temporary files
    pub cache_dir: PathBuf,
}

impl Default for AllSkillsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            default_scope: SkillScope::User,
            sources: Vec::new(),
            install_dir: PathBuf::from(".alltheskills"),
            cache_dir: PathBuf::from(".alltheskills/cache"),
        }
    }
}
