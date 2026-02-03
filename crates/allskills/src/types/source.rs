use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::SourceType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    Local {
        path: PathBuf,
    },
    GitHub {
        owner: String,
        repo: String,
        subdir: Option<String>,
        branch: Option<String>,
    },
    Remote {
        url: String,
        headers: Vec<(String, String)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: SourceType,
    pub enabled: bool,
    pub scope: SkillScope,
    pub priority: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillScope {
    Global,
    User,
    Project,
}
