use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource, SkillMetadata};
use crate::{Result, Error};

pub struct LocalProvider;

#[async_trait]
impl crate::providers::SkillProvider for LocalProvider {
    fn name(&self) -> &'static str {
        "Local Directory"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Local
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { .. })
    }

    async fn list_skills(&self, config: &crate::types::SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Local => std::env::current_dir()?,
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
        let SkillSource::Local { path: source_path } = source else {
            return Err(Error::Install { reason: "Invalid source type for Local provider".to_string() });
        };

        // Copy directory contents
        if source_path.is_dir() {
            std::fs::create_dir_all(&target)?;
            for entry in std::fs::read_dir(&source_path)? {
                let entry = entry?;
                let dest = target.join(entry.file_name());
                if entry.path().is_dir() {
                    copy_dir_all(entry.path(), dest)?;
                } else {
                    std::fs::copy(entry.path(), &dest)?;
                }
            }
        }

        // Parse the installed skill
        self.parse_skill_dir(target.clone()).await?
            .ok_or_else(|| Error::Install { reason: "Failed to parse installed skill".to_string() })
    }
}

impl LocalProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        let json_path = path.join("claude.json");
        let md_path = path.join("README.md");

        if json_path.exists() {
            self.parse_json(path, json_path).await
        } else if md_path.exists() {
            self.parse_markdown(path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value = serde_json::from_str(&content)?;

        let skill = Skill {
            id: config["name"].as_str().unwrap_or_default().to_string()
                .to_lowercase().replace(" ", "-"),
            name: config["name"].as_str().unwrap_or_default().to_string(),
            description: config["description"].as_str().unwrap_or_default().to_string(),
            version: config["version"].as_str().map(|s| s.to_string()),
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Local,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author: config["author"].as_str().map(|s| s.to_string()),
                tags: config["tags"].as_array().cloned().unwrap_or_default()
                    .iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
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
            description: "Local skill".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Local,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}

fn copy_dir_all(src: PathBuf, dst: PathBuf) -> Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
