use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for Roo Code (formerly Roo Cline) skills
///
/// Roo Code stores skills in `~/.roo/skills/` directory.
/// Skills can be defined via `roo.json` or `.roomodes` files.
pub struct RooProvider;

#[async_trait]
impl crate::providers::SkillProvider for RooProvider {
    fn name(&self) -> &'static str {
        "Roo Code Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::RooCode
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("roo"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::RooCode => {
                // Use home directory for Roo Code skills
                dirs::home_dir()
                    .map(|h| h.join(".roo/skills"))
                    .unwrap_or_else(|| PathBuf::from(".roo/skills"))
            }
            _ => return Ok(vec![]),
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
        // Try to read README.md first, then fall back to roo.json
        let readme_path = skill.path.join("README.md");
        if readme_path.exists() {
            let content = std::fs::read_to_string(&readme_path).map_err(Error::from)?;
            return Ok(content);
        }

        // Fall back to roo.json for content
        let json_path = skill.path.join("roo.json");
        if json_path.exists() {
            let content = std::fs::read_to_string(&json_path).map_err(Error::from)?;
            return Ok(content);
        }

        Err(Error::NotFound {
            name: skill.name.clone(),
        })
    }

    async fn install(&self, _source: SkillSource, _target: PathBuf) -> Result<Skill> {
        Err(Error::Install {
            reason: "Install not yet implemented for Roo Code provider".to_string(),
        })
    }
}

impl RooProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // Look for roo.json or .roomodes file
        let json_path = path.join("roo.json");
        let roomodes_path = path.join(".roomodes");
        let readme_path = path.join("README.md");

        if json_path.exists() {
            self.parse_roo_json(path, json_path).await
        } else if roomodes_path.exists() {
            self.parse_roomodes(path, roomodes_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_roo_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| Error::Parse {
                message: format!("Failed to parse roo.json: {}", e),
            })?;

        let name = config["name"].as_str().unwrap_or_default().to_string();
        let description = config["description"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let author = config["author"].as_str().map(|s| s.to_string());
        let version = config["version"].as_str().map(|s| s.to_string());

        let tags: Vec<String> = if let Some(tags_array) = config["tags"].as_array() {
            tags_array
                .iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect()
        } else {
            Vec::new()
        };

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::RooCode,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::RooSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_roomodes(&self, path: PathBuf, roomodes_path: PathBuf) -> Result<Option<Skill>> {
        // .roomodes is a JSON file containing custom modes for Roo Code
        let content = std::fs::read_to_string(&roomodes_path)?;
        let config: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| Error::Parse {
                message: format!("Failed to parse .roomodes: {}", e),
            })?;

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let description = config["description"]
            .as_str()
            .unwrap_or("Roo Code custom mode")
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::RooCode,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author: config["author"].as_str().map(|s| s.to_string()),
                tags: vec!["roo-mode".to_string()],
                ..Default::default()
            },
            format: SkillFormat::RooSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_markdown(&self, path: PathBuf) -> Result<Option<Skill>> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description: "Roo Code skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::RooCode,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
