//! Skill providers for different AI assistant platforms
//!
//! This module contains implementations of the [`SkillProvider`] trait
//! for various AI assistants and skill formats.

pub mod claude;
pub mod cline;
pub mod cloudflare;
pub mod codex;
pub mod cursor;
pub mod detect;
pub mod github;
pub mod kilo;
pub mod local;
pub mod moltbot;
pub mod openclaw;
pub mod roo;
pub mod trait_;
pub mod vercel;

pub use detect::KnownSources;
pub use trait_::SkillProvider;

// Re-export provider structs for convenience
pub use claude::ClaudeProvider;
pub use cline::ClineProvider;
pub use cloudflare::CloudflareProvider;
pub use codex::CodexProvider;
pub use cursor::CursorProvider;
pub use github::GitHubProvider;
pub use kilo::KiloProvider;
pub use local::LocalProvider;
pub use moltbot::MoltbotProvider;
pub use openclaw::OpenClawProvider;
pub use roo::RooProvider;
pub use vercel::VercelProvider;
