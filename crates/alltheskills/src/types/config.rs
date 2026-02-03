use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{SkillScope, SourceConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllSkillsConfig {
    pub version: u8,
    pub default_scope: SkillScope,
    pub sources: Vec<SourceConfig>,
    pub install_dir: PathBuf,
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
