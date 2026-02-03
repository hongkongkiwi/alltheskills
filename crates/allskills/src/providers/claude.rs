use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource, SkillMetadata, SourceConfig};
use crate::{Result, Error};

pub struct ClaudeProvider;

#[async_trait]
impl crate::providers::SkillProvider for ClaudeProvider {
    fn name(&self) -> &'static str {
        "Claude Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Claude
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("claude"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Claude => {
                // For Claude, we need to get the path from the skill source
                // This is a workaround - ideally SourceConfig should contain the actual path
                PathBuf::from(format!("/Users/andy/.claude/skills"))
            }
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(skill) = self.parse_skill_dir(entry.path()).await? {
                        skills.push(skill);
                    }
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String> {
        let readme_path = skill.path.join("README.md");
        let content = std::fs::read_to_string(&readme_path).map_err(|e| Error::from(e))?;
        Ok(content)
    }

    async fn install(&self, _source: SkillSource, _target: PathBuf) -> Result<Skill> {
        Err(Error::Install { reason: "Install not yet implemented for Claude provider".to_string() })
    }
}

impl ClaudeProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // Look for claude.json or skill.md file
        let json_path = path.join("claude.json");
        let md_path = path.join("skill.md");

        if json_path.exists() {
            self.parse_claude_json(path, json_path).await
        } else if md_path.exists() {
            self.parse_skill_md(path, md_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_claude_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| Error::Parse { message: format!("Failed to parse claude.json: {}", e) })?;

        let name = config["name"].as_str().unwrap_or_default().to_string();
        let description = config["description"].as_str().unwrap_or_default().to_string();
        let author = config["author"].as_str().map(|s| s.to_string());
        let version = config["version"].as_str().map(|s| s.to_string());

        let tags: Vec<String> = if let Some(tags_array) = config["tags"].as_array() {
            tags_array.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect()
        } else {
            Vec::new()
        };

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Claude,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::ClaudeSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_skill_md(&self, path: PathBuf, _md_path: PathBuf) -> Result<Option<Skill>> {
        // Parse markdown-based skill (older format)
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description: "Claude skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Claude,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
