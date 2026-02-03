//! Skill source detection utilities
//!
//! This module provides the [`KnownSources`] struct for detecting skill directories
//! for various AI assistant platforms.
//!
//! # Environment Variables
//!
//! Each detection method first checks for an environment variable override before
//! falling back to default paths:
//!
//! | Platform | Environment Variable | Default Path |
//! |----------|---------------------|--------------|
//! | Claude | `CLAUDE_SKILLS_DIR` | `~/.claude/skills` |
//! | Cline | `CLINE_SKILLS_DIR` | `~/.cline/skills` |
//! | Cursor | `CURSOR_RULES_DIR` | `~/.cursor/rules` |
//! | OpenClaw | `OPENCLAW_SKILLS_DIR` | `~/.openclaw/skills` |
//! | Roo Code | `ROO_SKILLS_DIR` | `~/.roo/skills` |
//! | Kilo Code | `KILO_SKILLS_DIR` | `~/.kilo/skills` |
//! | OpenAI Codex | `CODEX_SKILLS_DIR` | `~/.codex/skills` |
//! | Moltbot | `MOLTBOT_SKILLS_DIR` / `CLAWDBOT_SKILLS_DIR` | `~/.moltbot/skills` |
//! | Vercel | `VERCEL_SKILLS_DIR` | `~/.vercel/ai/skills` |
//! | Cloudflare | `CLOUDFLARE_SKILLS_DIR` | `~/.cloudflare/workers/skills` |
//!
//! # Example
//!
//! ```rust
//! use alltheskills::providers::KnownSources;
//!
//! // Check if Claude skills are installed
//! if let Some(path) = KnownSources::claude_skills_dir() {
//!     println!("Claude skills found at: {}", path.display());
//! }
//!
//! // Check for Moltbot (supports legacy ClawdBot paths)
//! if let Some(path) = KnownSources::moltbot_skills_dir() {
//!     println!("Moltbot skills found at: {}", path.display());
//! }
//! ```

use std::path::PathBuf;

/// Utility struct for detecting skill directories
///
/// Provides static methods to detect the installation directories for
/// various AI assistant platforms. Each method checks for environment
/// variable overrides before checking default paths.
pub struct KnownSources;

impl KnownSources {
    /// Detects Claude Code skills directory
    ///
    /// Checks `CLAUDE_SKILLS_DIR` env var, then `~/.claude/skills`,
    /// then `~/.claude/plugins/skills`.
    pub fn claude_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLAUDE_SKILLS_DIR",
            ["~/.claude/skills", "~/.claude/plugins/skills"],
        )
    }

    /// Detects Cline skills directory
    ///
    /// Checks `CLINE_SKILLS_DIR` env var, then `~/.cline/skills`.
    pub fn cline_skills_dir() -> Option<PathBuf> {
        Self::detect_path("CLINE_SKILLS_DIR", ["~/.cline/skills"])
    }

    /// Detects OpenClaw skills directory
    ///
    /// Checks `OPENCLAW_SKILLS_DIR` env var, then `~/.openclaw/skills`.
    pub fn openclaw_skills_dir() -> Option<PathBuf> {
        Self::detect_path("OPENCLAW_SKILLS_DIR", ["~/.openclaw/skills"])
    }

    /// Detects Roo Code skills directory
    ///
    /// Checks `ROO_SKILLS_DIR` env var, then `~/.roo/skills`.
    pub fn roo_skills_dir() -> Option<PathBuf> {
        Self::detect_path("ROO_SKILLS_DIR", ["~/.roo/skills"])
    }

    /// Detects Kilo Code skills directory
    ///
    /// Checks `KILO_SKILLS_DIR` env var, then `~/.kilo/skills`.
    pub fn kilo_skills_dir() -> Option<PathBuf> {
        Self::detect_path("KILO_SKILLS_DIR", ["~/.kilo/skills"])
    }

    /// Detects OpenAI Codex skills directory
    ///
    /// Checks `CODEX_SKILLS_DIR` env var, then `~/.codex/skills`.
    pub fn codex_skills_dir() -> Option<PathBuf> {
        Self::detect_path("CODEX_SKILLS_DIR", ["~/.codex/skills"])
    }

    /// Detects Vercel AI SDK skills directory
    ///
    /// Checks `VERCEL_SKILLS_DIR` env var, then `~/.vercel/ai/skills`,
    /// then `~/.ai/skills`.
    pub fn vercel_skills_dir() -> Option<PathBuf> {
        Self::detect_path("VERCEL_SKILLS_DIR", ["~/.vercel/ai/skills", "~/.ai/skills"])
    }

    /// Detects Cloudflare Workers AI skills directory
    ///
    /// Checks `CLOUDFLARE_SKILLS_DIR` env var, then `~/.cloudflare/workers/skills`,
    /// then `~/.workers-ai/skills`.
    pub fn cloudflare_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLOUDFLARE_SKILLS_DIR",
            ["~/.cloudflare/workers/skills", "~/.workers-ai/skills"],
        )
    }

    /// Detects Moltbot (formerly ClawdBot) skills directory
    ///
    /// Checks `MOLTBOT_SKILLS_DIR` env var, then `CLAWDBOT_SKILLS_DIR` for
    /// backward compatibility, then `~/.moltbot/skills`, then `~/.clawdbot/skills`.
    pub fn moltbot_skills_dir() -> Option<PathBuf> {
        // Check new name first, then legacy
        if let Ok(val) = std::env::var("MOLTBOT_SKILLS_DIR") {
            return Some(PathBuf::from(val));
        }
        if let Ok(val) = std::env::var("CLAWDBOT_SKILLS_DIR") {
            return Some(PathBuf::from(val));
        }
        // Check paths
        if let Ok(home) = std::env::var("HOME") {
            let moltbot_path = format!("{}/.moltbot/skills", home);
            if PathBuf::from(&moltbot_path).exists() {
                return Some(PathBuf::from(moltbot_path));
            }
            let clawdbot_path = format!("{}/.clawdbot/skills", home);
            if PathBuf::from(&clawdbot_path).exists() {
                return Some(PathBuf::from(clawdbot_path));
            }
        }
        None
    }

    /// Detects Cursor rules directory
    ///
    /// Checks `CURSOR_RULES_DIR` env var, then `~/.cursor/rules`,
    /// then `~/.cursor`.
    pub fn cursor_rules_dir() -> Option<PathBuf> {
        Self::detect_path("CURSOR_RULES_DIR", ["~/.cursor/rules", "~/.cursor"])
    }

    /// Generic path detection helper
    ///
    /// First checks the environment variable, then expands and checks
    /// each fallback path (supporting `~` for home directory).
    fn detect_path(
        env_key: &str,
        fallbacks: impl IntoIterator<Item = &'static str>,
    ) -> Option<PathBuf> {
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
