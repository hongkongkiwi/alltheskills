pub mod trait_;
pub mod detect;
pub mod claude;
pub mod cline;
pub mod openclaw;
pub mod roo;
pub mod github;
pub mod local;

pub use trait_::SkillProvider;
pub use detect::KnownSources;
