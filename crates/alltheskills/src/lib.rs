//! # AllTheSkills
//!
//! A Rust library for reading and installing AI skills from various sources
//! including Claude, Cline, OpenClaw, Vercel, Cloudflare, and more.
//!
//! ## Quick Start
//!
//! ```rust
//! use alltheskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AllSkillsConfig::default();
//!     let mut reader = SkillReader::new(config);
//!
//!     // Add providers for sources you want to read from
//!     reader.add_provider(ClaudeProvider);
//!     reader.add_provider(alltheskills::providers::LocalProvider);
//!
//!     // List all skills
//!     let skills = reader.list_all_skills().await?;
//!     println!("Found {} skill(s)", skills.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Unified Skill Format** - Read skills from multiple AI assistants with a single API
//! - **Multiple Providers** - Support for Claude, Cline, OpenClaw, Vercel, Cloudflare, Roo Code, and more
//! - **Extensible** - Trait-based provider architecture for adding new sources
//! - **Async** - Built on tokio for asynchronous operations
//!
//! ## Supported Sources
//!
//! | Source | Location | Format |
//! |--------|----------|--------|
//! | Claude Code | `~/.claude/skills/` | `claude.json`, `skill.md` |
//! | Cline | `~/.cline/skills/` | `cline.json`, `custom-instructions.md` |
//! | Cursor | `~/.cursor/rules/`, `.cursorrules` | `.cursorrules`, `cursor.json` |
//! | OpenClaw | `~/.openclaw/skills/` | `skill.json` |
//! | Vercel AI SDK | `~/.vercel/ai/skills/` | `skill.json`, `ai.config.json` |
//! | Cloudflare Workers AI | `~/.cloudflare/workers/skills/` | `worker.js/ts`, `wrangler.toml` |
//! | Roo Code | `~/.roo/skills/` | `roo.json`, `.roomodes` |
//! | Moltbot | `~/.moltbot/skills/` | `manifest.json`, `SKILL.md` |
//! | GitHub | Repository URLs | Any format |
//! | Local | Custom paths | Any format |

use futures::stream::{self, StreamExt};

pub mod core;
pub mod error;
pub mod providers;
pub mod types;

pub use error::Error;
pub use providers::{KnownSources, SkillProvider};
pub use types::{AllSkillsConfig, Skill, SkillScope, SourceType};

/// Result type alias for alltheskills operations
pub type Result<T> = std::result::Result<T, Error>;

/// Unified skill reader that queries all configured sources
///
/// The `SkillReader` is the main entry point for discovering and reading
/// skills from multiple sources. It aggregates multiple [`SkillProvider`]
/// implementations and provides unified access to all skills.
///
/// # Example
///
/// ```rust
/// use alltheskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};
///
/// async fn example() -> Result<(), alltheskills::Error> {
///     let config = AllSkillsConfig::default();
///     let mut reader = SkillReader::new(config);
///
///     reader.add_provider(ClaudeProvider);
///     reader.add_provider(alltheskills::providers::LocalProvider);
///
///     let skills = reader.list_all_skills().await?;
///     println!("Found {} skills", skills.len());
///
///     Ok(())
/// }
/// ```
pub struct SkillReader {
    _config: AllSkillsConfig,
    providers: Vec<Box<dyn crate::providers::SkillProvider>>,
}

impl SkillReader {
    /// Creates a new `SkillReader` with the given configuration
    pub fn new(config: AllSkillsConfig) -> Self {
        Self {
            _config: config,
            providers: Vec::new(),
        }
    }

    /// Adds a provider to the skill reader
    ///
    /// Providers are queried in parallel when listing skills.
    pub fn add_provider<P: crate::providers::SkillProvider + 'static>(&mut self, provider: P) {
        self.providers.push(Box::new(provider));
    }

    /// Lists all skills from all configured providers
    ///
    /// This method queries all registered providers concurrently and
    /// returns a combined list of all discovered skills.
    pub async fn list_all_skills(&self) -> Result<Vec<Skill>> {
        let futures = self.providers.iter().map(|p| async {
            let config = crate::types::SourceConfig {
                name: p.name().to_string(),
                source_type: SourceType::Local,
                enabled: true,
                scope: crate::types::SkillScope::User,
                priority: 0,
            };
            p.list_skills(&config).await
        });

        let results: Vec<Result<Vec<Skill>>> =
            stream::iter(futures).buffer_unordered(10).collect().await;

        let mut all_skills = Vec::new();
        for result in results {
            match result {
                Ok(skills) => all_skills.extend(skills),
                Err(e) => eprintln!("Failed to list skills: {}", e),
            }
        }

        Ok(all_skills)
    }

    /// Searches for skills matching the given predicate
    ///
    /// # Example
    ///
    /// ```rust
    /// use alltheskills::{SkillReader, AllSkillsConfig, providers::ClaudeProvider};
    ///
    /// async fn search_example() -> Result<(), alltheskills::Error> {
    ///     let config = AllSkillsConfig::default();
    ///     let mut reader = SkillReader::new(config);
    ///     reader.add_provider(ClaudeProvider);
    ///
    ///     let git_skills = reader
    ///         .search_skills(|s| s.name.to_lowercase().contains("git"))
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn search_skills<F>(&self, predicate: F) -> Result<Vec<Skill>>
    where
        F: Fn(&Skill) -> bool + Send + Sync,
    {
        let all = self.list_all_skills().await?;
        Ok(all.into_iter().filter(predicate).collect())
    }
}
