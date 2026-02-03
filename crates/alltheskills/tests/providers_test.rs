use alltheskills::providers::claude::ClaudeProvider;
use alltheskills::providers::cline::ClineProvider;
use alltheskills::providers::github::GitHubProvider;
use alltheskills::providers::local::LocalProvider;
use alltheskills::providers::moltbot::MoltbotProvider;
use alltheskills::providers::openclaw::OpenClawProvider;
use alltheskills::providers::roo::RooProvider;
use alltheskills::providers::{KnownSources, SkillProvider};
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
fn test_roo_skills_dir_returns_path() {
    // The function returns Option<PathBuf>, verify it's callable
    let result = KnownSources::roo_skills_dir();
    let _ = result;
}

#[test]
fn test_cline_skills_dir_returns_path() {
    // The function returns Option<PathBuf>, verify it's callable
    let result = KnownSources::cline_skills_dir();
    let _ = result;
}

#[test]
fn test_moltbot_skills_dir_returns_path() {
    // The function returns Option<PathBuf>, verify it's callable
    let result = KnownSources::moltbot_skills_dir();
    let _ = result;
}

#[test]
fn test_skill_provider_trait_objects() {
    // Verify providers can be converted to trait objects
    let _provider: Box<dyn SkillProvider> = Box::new(ClaudeProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(LocalProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(OpenClawProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(RooProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(ClineProvider);
    let _provider: Box<dyn SkillProvider> = Box::new(MoltbotProvider);
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
    assert_eq!(
        provider.source_type(),
        SourceType::Custom("openclaw".to_string())
    );
}

#[test]
fn test_roo_provider_source_type() {
    let provider = RooProvider;
    assert_eq!(provider.name(), "Roo Code Skills");
    assert_eq!(provider.source_type(), SourceType::RooCode);
}

#[test]
fn test_cline_provider_source_type() {
    let provider = ClineProvider;
    assert_eq!(provider.name(), "Cline Skills");
    assert_eq!(provider.source_type(), SourceType::Cline);
}

#[test]
fn test_moltbot_provider_source_type() {
    let provider = MoltbotProvider;
    assert_eq!(provider.name(), "Moltbot Skills");
    assert_eq!(provider.source_type(), SourceType::Moltbot);
}

#[test]
fn test_can_handle_local_source() {
    let provider = LocalProvider;
    let source = SkillSource::Local {
        path: PathBuf::from("/test/path"),
    };
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

#[test]
fn test_can_handle_roo_source() {
    let provider = RooProvider;
    let source = SkillSource::Local {
        path: PathBuf::from("/home/user/.roo/skills/my-skill"),
    };
    assert!(provider.can_handle(&source));
}

#[test]
fn test_can_handle_cline_source() {
    let provider = ClineProvider;
    let source = SkillSource::Local {
        path: PathBuf::from("/home/user/.cline/skills/my-skill"),
    };
    assert!(provider.can_handle(&source));
}

#[test]
fn test_can_handle_moltbot_source() {
    let provider = MoltbotProvider;
    let source = SkillSource::Local {
        path: PathBuf::from("/home/user/.moltbot/skills/my-skill"),
    };
    assert!(provider.can_handle(&source));
}

#[test]
fn test_can_handle_clawdbot_legacy_source() {
    let provider = MoltbotProvider;
    let source = SkillSource::Local {
        path: PathBuf::from("/home/user/.clawdbot/skills/my-skill"),
    };
    assert!(provider.can_handle(&source));
}
