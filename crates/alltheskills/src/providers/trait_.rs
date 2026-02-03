//! Provider trait definition for AllTheSkills
//!
//! This module defines the [`SkillProvider`] trait, which is the core abstraction
//! for implementing support for different AI assistant platforms.
//!
//! # Implementing a Provider
//!
//! To add support for a new skill source, implement the [`SkillProvider`] trait:
//!
//! ```rust
//! use async_trait::async_trait;
//! use alltheskills::{
//!     Skill, SkillProvider, 
//!     types::{SkillSource, SourceConfig, SourceType}
//! };
//!
//! pub struct MyProvider;
//!
//! #[async_trait]
//! impl SkillProvider for MyProvider {
//!     fn name(&self) -> &'static str {
//!         "My AI Skills"
//!     }
//!
//!     fn source_type(&self) -> SourceType {
//!         SourceType::Custom("my-ai".to_string())
//!     }
//!
//!     fn can_handle(&self, source: &SkillSource) -> bool {
//!         matches!(source, SkillSource::Local { path } 
//!             if path.to_string_lossy().contains("my-ai"))
//!     }
//!
//!     async fn list_skills(&self, config: &SourceConfig) -> alltheskills::Result<Vec<Skill>> {
//!         // Implementation
//!         Ok(vec![])
//!     }
//!
//!     async fn read_skill(&self, skill: &Skill) -> alltheskills::Result<String> {
//!         // Implementation
//!         Ok(String::new())
//!     }
//!
//!     async fn install(&self, source: SkillSource, target: std::path::PathBuf) 
//!         -> alltheskills::Result<Skill> {
//!         // Implementation
//!         unimplemented!()
//!     }
//! }
//! ```

use crate::types::{Skill, SkillSource, SourceConfig};
use async_trait::async_trait;

/// Trait for skill providers that can discover and read skills from a source
///
/// This is the core abstraction for adding support for new AI assistant platforms.
/// Each provider implements methods to:
/// - List available skills
/// - Read skill content
/// - Install skills to a target location
///
/// # Thread Safety
///
/// All providers must be `Send + Sync` as they may be used across async boundaries.
#[async_trait]
pub trait SkillProvider: Send + Sync {
    /// Returns the display name of this provider
    ///
    /// This is used for logging and user-facing output.
    fn name(&self) -> &'static str;

    /// Returns the source type this provider handles
    ///
    /// See [`SourceType`](crate::types::SourceType) for the list of supported types.
    fn source_type(&self) -> crate::types::SourceType;

    /// Checks if this provider can handle the given source
    ///
    /// Providers use this method to filter sources they can process.
    /// For example, a GitHub provider would return `true` for `SkillSource::GitHub`.
    fn can_handle(&self, source: &SkillSource) -> bool;

    /// Lists all skills available from this provider
    ///
    /// This method is called to discover skills from the configured source.
    /// It should return a list of [`Skill`] structs with complete metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the source cannot be accessed or parsed.
    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>, crate::Error>;

    /// Reads the content of a specific skill
    ///
    /// Given a [`Skill`] (typically returned from `list_skills`), this method
    /// reads the actual skill content (prompts, instructions, etc.).
    ///
    /// # Errors
    ///
    /// Returns an error if the skill cannot be read.
    async fn read_skill(&self, skill: &Skill) -> Result<String, crate::Error>;

    /// Installs a skill from a source to a target directory
    ///
    /// # Arguments
    ///
    /// * `source` - The source to install from
    /// * `target` - The target directory to install to
    ///
    /// # Errors
    ///
    /// Returns an error if installation fails.
    async fn install(
        &self,
        source: SkillSource,
        target: std::path::PathBuf,
    ) -> Result<Skill, crate::Error>;
}
