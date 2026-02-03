use alltheskills::providers::{KnownSources, SkillProvider};
use alltheskills::providers::claude::ClaudeProvider;
use alltheskills::providers::local::LocalProvider;
use alltheskills::providers::openclaw::OpenClawProvider;
use alltheskills::providers::github::GitHubProvider;
use alltheskills::types::{SkillSource, SourceType};
use std::path::PathBuf;

#[test]
fn test_known_sources_struct_exists() {
    // Verify KnownSources can be constructed
    let _ = KnownSources;
}

#[test]
fn test_claude_skills_dir_returns_path() {
    // The function returns Option<PathBuf>, verify it's callable
    let result = KnownSources::claude_skills_dir();
    // Just verify it returns something (may be None if env vars not set)
    let _ = result;
}

#[test]
fn test_skill_provider_trait_objects() {
    // Verify providers can be converted to trait objects
    let _provider: Box<dyn SkillProvider> = Box::new(ClaudeProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(LocalProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(OpenClawProvider);
}

#[test]
fn test_claude_provider_source_type() {
    let provider = ClaudeProvider;
    assert_eq!(provider.name(), "Claude Skills");
    assert_eq!(provider.source_type(), SourceType::Claude);
}

#[test]
fn test_local_provider_source_type() {
    let provider = LocalProvider;
    assert_eq!(provider.name(), "Local Directory");
    assert_eq!(provider.source_type(), SourceType::Local);
}

#[test]
fn test_openclaw_provider_source_type() {
    let provider = OpenClawProvider;
    assert_eq!(provider.name(), "OpenClaw Skills");
    assert_eq!(provider.source_type(), SourceType::Custom("openclaw".to_string()));
}

#[test]
fn test_can_handle_local_source() {
    let provider = LocalProvider;
    let source = SkillSource::Local { path: PathBuf::from("/test/path") };
    assert!(provider.can_handle(&source));
}

#[test]
fn test_can_handle_github_source() {
    let provider = GitHubProvider;
    let source = SkillSource::GitHub {
        owner: "test".to_string(),
        repo: "test".to_string(),
        subdir: None,
        branch: None,
    };
    assert!(provider.can_handle(&source));
}
