//! Core types for the AllTheSkills library
//!
//! This module defines the fundamental data structures used throughout
//! the library for representing skills, sources, and configurations.

pub mod config;
pub mod skill;
pub mod source;

pub use config::AllSkillsConfig;
pub use skill::{Skill, SkillFormat, SkillMetadata, SourceType};
pub use source::{SkillScope, SkillSource, SourceConfig};
