use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::utils::copy_skill_dir;
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for OpenAI Codex skills
///
/// OpenAI Codex stores skills in `~/.codex/skills/` directory.
/// Skills are typically defined via JSON configuration files.
///
/// # Skill Structure
/// ```text
/// ~/.codex/skills/my-skill/
/// ├── codex.json       # Skill configuration
/// ├── instructions.md  # Main skill instructions (optional)
/// └── README.md        # Documentation (optional)
/// ```
pub struct CodexProvider;

#[async_trait]
impl crate::providers::SkillProvider for CodexProvider {
    fn name(&self) -> &'static str {
        "OpenAI Codex Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::OpenAICodex
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("codex"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::OpenAICodex => {
                // Use home directory for Codex skills
                dirs::home_dir()
                    .map(|h| h.join(".codex/skills"))
                    .unwrap_or_else(|| PathBuf::from(".codex/skills"))
            }
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&path) {
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
        // Try instructions.md first, then README.md, then codex.json
        let instructions_path = skill.path.join("instructions.md");
        if instructions_path.exists() {
            return std::fs::read_to_string(&instructions_path).map_err(Error::from);
        }

        let readme_path = skill.path.join("README.md");
        if readme_path.exists() {
            return std::fs::read_to_string(&readme_path).map_err(Error::from);
        }

        let codex_json = skill.path.join("codex.json");
        if codex_json.exists() {
            return std::fs::read_to_string(&codex_json).map_err(Error::from);
        }

        Err(Error::NotFound {
            name: skill.name.clone(),
        })
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill> {
        let source_path = match &source {
            SkillSource::Local { path } => path.clone(),
            _ => {
                return Err(Error::Install {
                    reason: "OpenAI Codex provider only supports local installation".to_string(),
                })
            }
        };

        std::fs::create_dir_all(&target)?;
        copy_skill_dir(&source_path, &target)?;

        self.parse_skill_dir(target.clone())
            .await?
            .ok_or_else(|| Error::Install {
                reason: "Failed to parse installed OpenAI Codex skill".to_string(),
            })
    }
}

impl CodexProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // Look for codex.json or instructions.md file
        let json_path = path.join("codex.json");
        let instructions_path = path.join("instructions.md");
        let readme_path = path.join("README.md");

        if json_path.exists() {
            self.parse_codex_json(path, json_path).await
        } else if instructions_path.exists() {
            self.parse_instructions_md(path, instructions_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path, readme_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_codex_json(
        &self,
        path: PathBuf,
        json_path: PathBuf,
    ) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value = serde_json::from_str(&content).map_err(|e| Error::Parse {
            message: format!("Failed to parse codex.json: {}", e),
        })?;

        let name = config["name"].as_str().unwrap_or_default().to_string();
        let description = config["description"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let version = config["version"].as_str().map(|s| s.to_string());
        let author = config["author"].as_str().map(|s| s.to_string());

        // Extract model preferences for tags
        let mut tags = Vec::new();
        if let Some(model) = config["model"].as_str() {
            tags.push(format!("model:{}", model));
        }
        if let Some(tools) = config["tools"].as_array() {
            for tool in tools {
                if let Some(tool_name) = tool.as_str() {
                    tags.push(format!("tool:{}", tool_name));
                }
            }
        }

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::OpenAICodex,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::CodexSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_instructions_md(
        &self,
        path: PathBuf,
        instructions_path: PathBuf,
    ) -> Result<Option<Skill>> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        // Read first non-empty, non-header line as description
        let description = std::fs::read_to_string(&instructions_path)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "OpenAI Codex skill".to_string());

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::OpenAICodex,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags: vec!["instructions".to_string()],
                ..Default::default()
            },
            format: SkillFormat::CodexSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_markdown(&self, path: PathBuf, _readme_path: PathBuf) -> Result<Option<Skill>> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description: "OpenAI Codex skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::OpenAICodex,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
