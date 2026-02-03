use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::utils::copy_skill_dir;
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct VercelProvider;

#[async_trait]
impl crate::providers::SkillProvider for VercelProvider {
    fn name(&self) -> &'static str {
        "Vercel AI SDK Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Custom("vercel".to_string())
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if
            path.to_string_lossy().contains("vercel") ||
            path.to_string_lossy().contains(".ai"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        // Check if this source type matches our provider
        let is_our_type = match &config.source_type {
            SourceType::Custom(name) => name == "vercel",
            _ => false,
        };

        if !is_our_type {
            return Ok(vec![]);
        }

        // Get path from KnownSources or return empty for now
        let path = match crate::providers::KnownSources::vercel_skills_dir() {
            Some(p) => p,
            None => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir()
                    && let Some(skill) = self.parse_skill_dir(entry.path()).await?
                {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String> {
        let readme_path = skill.path.join("README.md");
        let content = std::fs::read_to_string(&readme_path)?;
        Ok(content)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill> {
        let source_path = match &source {
            SkillSource::Local { path } => path.clone(),
            _ => {
                return Err(Error::Install {
                    reason: "Vercel provider only supports local installation".to_string(),
                })
            }
        };

        std::fs::create_dir_all(&target)?;
        copy_skill_dir(&source_path, &target)?;

        self.parse_skill_dir(target.clone())
            .await?
            .ok_or_else(|| Error::Install {
                reason: "Failed to parse installed Vercel skill".to_string(),
            })
    }
}

impl VercelProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        let json_path = path.join("skill.json");
        let config_path = path.join("ai.config.json");

        if json_path.exists() {
            self.parse_skill_json(path, json_path).await
        } else if config_path.exists() {
            self.parse_ai_config(path, config_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_skill_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value = serde_json::from_str(&content)?;

        // Parse tags array safely
        let tags: Vec<String> = config["tags"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let skill = Skill {
            id: config["id"].as_str().unwrap_or_default().to_string(),
            name: config["name"].as_str().unwrap_or_default().to_string(),
            description: config["description"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            version: config["version"].as_str().map(|s| s.to_string()),
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("vercel".to_string()),
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author: config["author"].as_str().map(|s| s.to_string()),
                tags,
                repository: config["repository"].as_str().map(|s| s.to_string()),
                ..Default::default()
            },
            format: SkillFormat::GenericJson,
        };

        Ok(Some(skill))
    }

    async fn parse_ai_config(&self, path: PathBuf, config_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&config_path)?;
        let config: serde_json::Value = serde_json::from_str(&content)?;

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: config["name"].as_str().unwrap_or(&name).to_string(),
            description: config["description"]
                .as_str()
                .unwrap_or("Vercel AI skill")
                .to_string(),
            version: config["version"].as_str().map(|s| s.to_string()),
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("vercel".to_string()),
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericJson,
        };

        Ok(Some(skill))
    }
}
