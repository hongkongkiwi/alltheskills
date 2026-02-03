use std::path::{Path, PathBuf};
use crate::types::SourceType;

pub struct KnownSources;

impl KnownSources {
    pub fn claude_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLAUDE_SKILLS_DIR",
            ["~/.claude/skills", "~/.claude/plugins/skills"],
        )
    }

    pub fn cline_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLINE_SKILLS_DIR",
            ["~/.cline/skills"],
        )
    }

    pub fn openclaw_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "OPENCLAW_SKILLS_DIR",
            ["~/.openclaw/skills"],
        )
    }

    pub fn roo_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "ROO_SKILLS_DIR",
            ["~/.roo/skills"],
        )
    }

    pub fn kilo_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "KILO_SKILLS_DIR",
            ["~/.kilo/skills"],
        )
    }

    pub fn codex_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CODEX_SKILLS_DIR",
            ["~/.codex/skills"],
        )
    }

    fn detect_path(env_key: &str, fallbacks: impl IntoIterator<Item = &'static str>) -> Option<PathBuf> {
        // Check environment variable first
        if let Ok(val) = std::env::var(env_key) {
            return Some(PathBuf::from(val));
        }

        // Try home directory expansion for fallbacks
        if let Ok(home) = std::env::var("HOME") {
            for fallback in fallbacks {
                if let Some(path) = fallback.strip_prefix("~/") {
                    let expanded = format!("{}/{}", home, path);
                    if PathBuf::from(&expanded).exists() {
                        return Some(PathBuf::from(expanded));
                    }
                }
            }
        }

        None
    }
}
