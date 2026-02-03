use std::path::PathBuf;

pub struct KnownSources;

impl KnownSources {
    pub fn claude_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLAUDE_SKILLS_DIR",
            ["~/.claude/skills", "~/.claude/plugins/skills"],
        )
    }

    pub fn cline_skills_dir() -> Option<PathBuf> {
        Self::detect_path("CLINE_SKILLS_DIR", ["~/.cline/skills"])
    }

    pub fn openclaw_skills_dir() -> Option<PathBuf> {
        Self::detect_path("OPENCLAW_SKILLS_DIR", ["~/.openclaw/skills"])
    }

    pub fn roo_skills_dir() -> Option<PathBuf> {
        Self::detect_path("ROO_SKILLS_DIR", ["~/.roo/skills"])
    }

    pub fn kilo_skills_dir() -> Option<PathBuf> {
        Self::detect_path("KILO_SKILLS_DIR", ["~/.kilo/skills"])
    }

    pub fn codex_skills_dir() -> Option<PathBuf> {
        Self::detect_path("CODEX_SKILLS_DIR", ["~/.codex/skills"])
    }

    pub fn vercel_skills_dir() -> Option<PathBuf> {
        Self::detect_path("VERCEL_SKILLS_DIR", ["~/.vercel/ai/skills", "~/.ai/skills"])
    }

    pub fn cloudflare_skills_dir() -> Option<PathBuf> {
        Self::detect_path(
            "CLOUDFLARE_SKILLS_DIR",
            ["~/.cloudflare/workers/skills", "~/.workers-ai/skills"],
        )
    }

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

    pub fn cursor_rules_dir() -> Option<PathBuf> {
        Self::detect_path("CURSOR_RULES_DIR", ["~/.cursor/rules", "~/.cursor"])
    }

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
