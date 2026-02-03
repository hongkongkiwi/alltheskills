use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource, SkillMetadata};
use crate::{Result, Error};

pub struct OpenClawProvider;

#[async_trait]
impl crate::providers::SkillProvider for OpenClawProvider {
    fn name(&self) -> &'static str {
        "OpenClaw Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Custom("openclaw".to_string())
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("openclaw"))
    }

    async fn list_skills(&self, config: &crate::types::SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Custom(name) if name == "openclaw" => {
                // Need to get path from somewhere - using current dir as fallback
                std::env::current_dir()?
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

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill> {
        Err(Error::Install { reason: "Install not yet implemented for OpenClaw provider".to_string() })
    }
}

impl OpenClawProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // OpenClaw uses skill.json format
        let json_path = path.join("skill.json");
        let md_path = path.join("README.md");

        if json_path.exists() {
            self.parse_skill_json(path, json_path).await
        } else if md_path.exists() {
            self.parse_markdown(path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_skill_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value = serde_json::from_str(&content)?;

        let skill = Skill {
            id: config["id"].as_str().unwrap_or_default().to_string(),
            name: config["name"].as_str().unwrap_or_default().to_string(),
            description: config["description"].as_str().unwrap_or_default().to_string(),
            version: config["version"].as_str().map(|s| s.to_string()),
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("openclaw".to_string()),
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author: config["author"].as_str().map(|s| s.to_string()),
                tags: config["tags"].as_array().cloned().unwrap_or_default()
                    .iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
                repository: config["repository"].as_str().map(|s| s.to_string()),
                homepage: config["homepage"].as_str().map(|s| s.to_string()),
                license: config["license"].as_str().map(|s| s.to_string()),
                ..Default::default()
            },
            format: SkillFormat::GenericJson,
        };

        Ok(Some(skill))
    }

    async fn parse_markdown(&self, path: PathBuf) -> Result<Option<Skill>> {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description: "OpenClaw skill".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("openclaw".to_string()),
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
