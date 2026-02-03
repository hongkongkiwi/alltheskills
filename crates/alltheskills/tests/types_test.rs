use alltheskills::Skill;
use alltheskills::types::{SkillFormat, SourceType, SkillMetadata, SkillScope, SkillSource};
use std::path::PathBuf;

#[test]
fn test_skill_creation() {
    let skill = Skill {
        id: "test-skill".to_string(),
        name: "Test Skill".to_string(),
        description: "A test skill".to_string(),
        version: Some("1.0.0".to_string()),
        source: SkillSource::Local {
            path: PathBuf::from("/test"),
        },
        source_type: SourceType::Local,
        path: PathBuf::from("/test/skill"),
        installed_at: chrono::Utc::now(),
        metadata: SkillMetadata::default(),
        format: SkillFormat::GenericMarkdown,
    };

    assert_eq!(skill.name, "Test Skill");
    assert_eq!(skill.id, "test-skill");
    assert_eq!(skill.source_type, SourceType::Local);
    assert_eq!(skill.format, SkillFormat::GenericMarkdown);
}

#[test]
fn test_skill_metadata_default() {
    let metadata = SkillMetadata::default();
    assert!(metadata.author.is_none());
    assert!(metadata.tags.is_empty());
    assert!(metadata.homepage.is_none());
    assert!(metadata.repository.is_none());
    assert!(metadata.license.is_none());
}

#[test]
fn test_skill_scope_ordering() {
    // Verify scopes exist and have expected values
    assert_eq!(format!("{:?}", SkillScope::Global), "Global");
    assert_eq!(format!("{:?}", SkillScope::User), "User");
    assert_eq!(format!("{:?}", SkillScope::Project), "Project");
}

#[test]
fn test_skill_format_variants() {
    // Verify all format variants exist
    assert_eq!(format!("{:?}", SkillFormat::ClaudeSkill), "ClaudeSkill");
    assert_eq!(format!("{:?}", SkillFormat::ClaudePlugin), "ClaudePlugin");
    assert_eq!(format!("{:?}", SkillFormat::GenericMarkdown), "GenericMarkdown");
    assert_eq!(format!("{:?}", SkillFormat::GenericJson), "GenericJson");
    assert_eq!(format!("{:?}", SkillFormat::Unknown), "Unknown");
}

#[test]
fn test_source_type_variants() {
    assert_eq!(format!("{:?}", SourceType::Claude), "Claude");
    assert_eq!(format!("{:?}", SourceType::Cline), "Cline");
    assert_eq!(format!("{:?}", SourceType::GitHub), "GitHub");
    assert_eq!(format!("{:?}", SourceType::Local), "Local");
}

#[test]
fn test_skill_source_local() {
    let path = PathBuf::from("/some/path");
    let source = SkillSource::Local { path: path.clone() };

    match source {
        SkillSource::Local { path: p } => assert_eq!(p, path),
        _ => panic!("Expected Local variant"),
    }
}

#[test]
fn test_skill_source_github() {
    let source = SkillSource::GitHub {
        owner: "test-owner".to_string(),
        repo: "test-repo".to_string(),
        subdir: Some("skills/my-skill".to_string()),
        branch: Some("main".to_string()),
    };

    match source {
        SkillSource::GitHub { owner, repo, subdir, branch } => {
            assert_eq!(owner, "test-owner");
            assert_eq!(repo, "test-repo");
            assert_eq!(subdir, Some("skills/my-skill".to_string()));
            assert_eq!(branch, Some("main".to_string()));
        }
        _ => panic!("Expected GitHub variant"),
    }
}

#[test]
fn test_skill_source_remote() {
    let source = SkillSource::Remote {
        url: "https://example.com/skill.json".to_string(),
        headers: vec![("Authorization".to_string(), "Bearer token".to_string())],
    };

    match source {
        SkillSource::Remote { url, headers } => {
            assert_eq!(url, "https://example.com/skill.json");
            assert_eq!(headers.len(), 1);
        }
        _ => panic!("Expected Remote variant"),
    }
}
