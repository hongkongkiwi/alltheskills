//! Integration tests for skill providers

use alltheskills::{
    providers::*,
    types::{AllSkillsConfig, SourceConfig, SourceType},
    SkillReader,
};

/// Test that all providers can be created and added to a SkillReader
#[test]
fn test_all_providers_can_be_created() {
    let _claude = ClaudeProvider;
    let _cline = ClineProvider;
    let _cursor = CursorProvider;
    let _openclaw = OpenClawProvider;
    let _roo = RooProvider;
    let _moltbot = MoltbotProvider;
    let _codex = CodexProvider;
    let _kilo = KiloProvider;
    let _vercel = VercelProvider;
    let _cloudflare = CloudflareProvider;
    let _github = GitHubProvider;
    let _local = LocalProvider;
}

/// Test that providers return correct source types
#[test]
fn test_provider_source_types() {
    assert!(matches!(ClaudeProvider.source_type(), SourceType::Claude));
    assert!(matches!(ClineProvider.source_type(), SourceType::Cline));
    assert!(matches!(CursorProvider.source_type(), SourceType::Cursor));
    assert!(matches!(RooProvider.source_type(), SourceType::RooCode));
    assert!(matches!(CodexProvider.source_type(), SourceType::OpenAICodex));
    assert!(matches!(KiloProvider.source_type(), SourceType::KiloCode));
    assert!(matches!(MoltbotProvider.source_type(), SourceType::Moltbot));
    assert!(matches!(LocalProvider.source_type(), SourceType::Local));
    assert!(matches!(GitHubProvider.source_type(), SourceType::GitHub));
}

/// Test that providers have correct names
#[test]
fn test_provider_names() {
    assert_eq!(ClaudeProvider.name(), "Claude Skills");
    assert_eq!(ClineProvider.name(), "Cline Skills");
    assert_eq!(CursorProvider.name(), "Cursor Rules");
    assert_eq!(OpenClawProvider.name(), "OpenClaw Skills");
    assert_eq!(RooProvider.name(), "Roo Code Skills");
    assert_eq!(MoltbotProvider.name(), "Moltbot Skills");
    assert_eq!(CodexProvider.name(), "OpenAI Codex Skills");
    assert_eq!(KiloProvider.name(), "Kilo Code Skills");
    assert_eq!(VercelProvider.name(), "Vercel AI SDK Skills");
    assert_eq!(CloudflareProvider.name(), "Cloudflare Workers AI Skills");
    assert_eq!(GitHubProvider.name(), "GitHub Repository");
    assert_eq!(LocalProvider.name(), "Local Directory");
}

/// Test that SkillReader can be created and hold providers
#[test]
fn test_skill_reader_creation() {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    reader.add_provider(ClaudeProvider);
    reader.add_provider(ClineProvider);
    reader.add_provider(LocalProvider);

    // If we get here without panicking, the test passes
}

/// Test that empty skill lists are handled correctly
#[tokio::test]
async fn test_empty_skill_list() {
    let config = AllSkillsConfig::default();
    let reader = SkillReader::new(config);

    let skills = reader.list_all_skills().await;
    assert!(skills.is_ok());
    let skills = skills.unwrap();
    assert!(skills.is_empty());
}

/// Test SourceConfig creation
#[test]
fn test_source_config_creation() {
    let config = SourceConfig {
        name: "test-source".to_string(),
        source_type: SourceType::Claude,
        enabled: true,
        scope: alltheskills::types::SkillScope::User,
        priority: 0,
    };

    assert_eq!(config.name, "test-source");
    assert!(config.enabled);
    assert_eq!(config.priority, 0);
}

/// Test skill metadata defaults
#[test]
fn test_skill_metadata_defaults() {
    let metadata = alltheskills::types::SkillMetadata::default();

    assert!(metadata.author.is_none());
    assert!(metadata.tags.is_empty());
    assert!(metadata.homepage.is_none());
    assert!(metadata.repository.is_none());
    assert!(metadata.license.is_none());
    assert!(metadata.readme.is_none());
    assert!(metadata.requirements.is_empty());
}

/// Test SkillScope variants
#[test]
fn test_skill_scope_variants() {
    use alltheskills::types::SkillScope;

    let _global = SkillScope::Global;
    let _user = SkillScope::User;
    let _project = SkillScope::Project;
}

/// Test AllSkillsConfig defaults
#[test]
fn test_config_defaults() {
    let config = AllSkillsConfig::default();

    assert_eq!(config.version, 1);
    assert!(matches!(config.default_scope, alltheskills::types::SkillScope::User));
    assert!(config.sources.is_empty());
    assert_eq!(config.install_dir, std::path::PathBuf::from(".alltheskills"));
    assert_eq!(config.cache_dir, std::path::PathBuf::from(".alltheskills/cache"));
}
