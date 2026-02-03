use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::utils::copy_skill_dir;
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for Cline skills
///
/// Cline stores skills in `~/.cline/skills/` directory.
/// Skills are typically defined via `cline.json` configuration files
/// or as markdown-based custom instructions.
pub struct ClineProvider;

#[async_trait]
impl crate::providers::SkillProvider for ClineProvider {
    fn name(&self) -> &'static str {
        "Cline Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Cline
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("cline"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Cline => {
                // Use home directory for Cline skills
                dirs::home_dir()
                    .map(|h| h.join(".cline/skills"))
                    .unwrap_or_else(|| PathBuf::from(".cline/skills"))
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
        // Try to read README.md first, then fall back to cline.json
        let readme_path = skill.path.join("README.md");
        if readme_path.exists() {
            let content = std::fs::read_to_string(&readme_path).map_err(Error::from)?;
            return Ok(content);
        }

        // Fall back to cline.json for content
        let json_path = skill.path.join("cline.json");
        if json_path.exists() {
            let content = std::fs::read_to_string(&json_path).map_err(Error::from)?;
            return Ok(content);
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
                    reason: "Cline provider only supports local installation".to_string(),
                })
            }
        };

        std::fs::create_dir_all(&target)?;
        copy_skill_dir(&source_path, &target)?;

        self.parse_skill_dir(target.clone())
            .await?
            .ok_or_else(|| Error::Install {
                reason: "Failed to parse installed Cline skill".to_string(),
            })
    }
}

impl ClineProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // Look for cline.json or custom-instructions.md file
        let json_path = path.join("cline.json");
        let instructions_path = path.join("custom-instructions.md");
        let readme_path = path.join("README.md");

        if json_path.exists() {
            self.parse_cline_json(path, json_path).await
        } else if instructions_path.exists() {
            self.parse_instructions(path, instructions_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_cline_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| Error::Parse {
                message: format!("Failed to parse cline.json: {}", e),
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
            source_type: SourceType::Cline,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::ClineSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_instructions(
        &self,
        path: PathBuf,
        instructions_path: PathBuf,
    ) -> Result<Option<Skill>> {
        // Cline uses custom-instructions.md for custom instructions
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        // Read first line as description if available
        let description = std::fs::read_to_string(&instructions_path)
            .ok()
            .and_then(|content| content.lines().next().map(|s| s.to_string()))
            .unwrap_or_else(|| "Cline custom instructions".to_string());

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Cline,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags: vec!["custom-instructions".to_string()],
                ..Default::default()
            },
            format: SkillFormat::ClineSkill,
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
            description: "Cline skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Cline,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
