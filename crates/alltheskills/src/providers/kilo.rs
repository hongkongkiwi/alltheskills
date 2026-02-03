use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::utils::copy_skill_dir;
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for Kilo Code skills
///
/// Kilo Code stores skills in `~/.kilo/skills/` directory.
/// Skills are typically defined via YAML configuration files with markdown instructions.
///
/// # Skill Structure
/// ```text
/// ~/.kilo/skills/my-skill/
/// ├── kilo.yaml        # Skill configuration (YAML format)
/// ├── kilo.yml         # Alternative YAML extension
/// ├── instructions.md  # Main skill instructions (optional)
/// └── README.md        # Documentation (optional)
/// ```
pub struct KiloProvider;

#[async_trait]
impl crate::providers::SkillProvider for KiloProvider {
    fn name(&self) -> &'static str {
        "Kilo Code Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::KiloCode
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if path.to_string_lossy().contains("kilo"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::KiloCode => {
                // Use home directory for Kilo Code skills
                dirs::home_dir()
                    .map(|h| h.join(".kilo/skills"))
                    .unwrap_or_else(|| PathBuf::from(".kilo/skills"))
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
        // Try instructions.md first, then README.md, then kilo.yaml/kilo.yml
        let instructions_path = skill.path.join("instructions.md");
        if instructions_path.exists() {
            return std::fs::read_to_string(&instructions_path).map_err(Error::from);
        }

        let readme_path = skill.path.join("README.md");
        if readme_path.exists() {
            return std::fs::read_to_string(&readme_path).map_err(Error::from);
        }

        let kilo_yaml = skill.path.join("kilo.yaml");
        if kilo_yaml.exists() {
            return std::fs::read_to_string(&kilo_yaml).map_err(Error::from);
        }

        let kilo_yml = skill.path.join("kilo.yml");
        if kilo_yml.exists() {
            return std::fs::read_to_string(&kilo_yml).map_err(Error::from);
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
                    reason: "Kilo Code provider only supports local installation".to_string(),
                })
            }
        };

        std::fs::create_dir_all(&target)?;
        copy_skill_dir(&source_path, &target)?;

        self.parse_skill_dir(target.clone())
            .await?
            .ok_or_else(|| Error::Install {
                reason: "Failed to parse installed Kilo Code skill".to_string(),
            })
    }
}

impl KiloProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        // Look for kilo.yaml, kilo.yml, or instructions.md file
        let yaml_path = path.join("kilo.yaml");
        let yml_path = path.join("kilo.yml");
        let instructions_path = path.join("instructions.md");
        let readme_path = path.join("README.md");

        if yaml_path.exists() {
            self.parse_kilo_yaml(path, yaml_path).await
        } else if yml_path.exists() {
            self.parse_kilo_yaml(path, yml_path).await
        } else if instructions_path.exists() {
            self.parse_instructions_md(path, instructions_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path, readme_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_kilo_yaml(
        &self,
        path: PathBuf,
        yaml_path: PathBuf,
    ) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&yaml_path)?;
        let config: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| Error::Parse {
            message: format!("Failed to parse kilo.yaml: {}", e),
        })?;

        let name = config["name"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let description = config["description"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let version = config["version"].as_str().map(|s| s.to_string());
        let author = config["author"].as_str().map(|s| s.to_string());

        // Extract tags from config
        let mut tags = Vec::new();
        if let Some(tags_array) = config["tags"].as_sequence() {
            for tag in tags_array {
                if let Some(tag_str) = tag.as_str() {
                    tags.push(tag_str.to_string());
                }
            }
        }

        // Extract language for tags
        if let Some(language) = config["language"].as_str() {
            tags.push(format!("lang:{}", language));
        }

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::KiloCode,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::KiloSkill,
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
            .unwrap_or_else(|| "Kilo Code skill".to_string());

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::KiloCode,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags: vec!["instructions".to_string()],
                ..Default::default()
            },
            format: SkillFormat::KiloSkill,
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
            description: "Kilo Code skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::KiloCode,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
