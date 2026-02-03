pub mod core;
pub mod providers;
pub mod types;
pub mod error;

pub use types::{Skill, SourceType, SkillScope, AllSkillsConfig};
pub use providers::{SkillProvider, KnownSources};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
