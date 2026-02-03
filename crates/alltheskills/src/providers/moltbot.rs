use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for Moltbot (formerly ClawdBot) skills
///
/// Moltbot stores skills in `~/.moltbot/skills/` (formerly `~/.clawdbot/skills/`).
/// Skills are defined via:
/// - `manifest.json` - Skill metadata and configuration
/// - `SKILL.md` - Main skill instructions/prompt
/// - `index.ts` or main logic file - Implementation
///
/// # Skill Structure
/// ```text
/// ~/.moltbot/skills/my-skill/
/// ├── manifest.json    # Skill manifest with name, version, commands
/// ├── SKILL.md         # Main skill instructions
/// ├── index.ts         # Implementation (optional)
/// └── README.md        # Documentation (optional)
/// ```
pub struct MoltbotProvider;

#[async_trait]
impl crate::providers::SkillProvider for MoltbotProvider {
    fn name(&self) -> &'static str {
        "Moltbot Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Moltbot
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if
            path.to_string_lossy().contains("moltbot") ||
            path.to_string_lossy().contains("clawdbot")
        )
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Moltbot => dirs::home_dir()
                .map(|h| h.join(".moltbot/skills"))
                .or_else(|| dirs::home_dir().map(|h| h.join(".clawdbot/skills")))
                .unwrap_or_else(|| PathBuf::from(".moltbot/skills")),
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

        // Also check legacy clawdbot path if different
        let clawdbot_path = dirs::home_dir().map(|h| h.join(".clawdbot/skills"));
        if let Some(legacy_path) = clawdbot_path
            && legacy_path != path
            && legacy_path.exists()
            && let Ok(entries) = std::fs::read_dir(&legacy_path)
        {
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
        // Try SKILL.md first, then README.md, then manifest.json
        let skill_md = skill.path.join("SKILL.md");
        if skill_md.exists() {
            return std::fs::read_to_string(&skill_md).map_err(Error::from);
        }

        let readme_path = skill.path.join("README.md");
        if readme_path.exists() {
            return std::fs::read_to_string(&readme_path).map_err(Error::from);
        }

        let manifest_path = skill.path.join("manifest.json");
        if manifest_path.exists() {
            return std::fs::read_to_string(&manifest_path).map_err(Error::from);
        }

        Err(Error::NotFound {
            name: skill.name.clone(),
        })
    }

    async fn install(&self, _source: SkillSource, _target: PathBuf) -> Result<Skill> {
        Err(Error::Install {
            reason: "Install not yet implemented for Moltbot provider".to_string(),
        })
    }
}

impl MoltbotProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        let manifest_path = path.join("manifest.json");
        let skill_md_path = path.join("SKILL.md");
        let readme_path = path.join("README.md");

        if manifest_path.exists() {
            self.parse_manifest(path, manifest_path).await
        } else if skill_md_path.exists() {
            self.parse_skill_md(path, skill_md_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path, readme_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_manifest(&self, path: PathBuf, manifest_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| Error::Parse {
                message: format!("Failed to parse manifest.json: {}", e),
            })?;

        let name = manifest["name"].as_str().unwrap_or_default().to_string();
        let description = manifest["description"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let version = manifest["version"].as_str().map(|s| s.to_string());
        let author = manifest["author"].as_str().map(|s| s.to_string());

        // Extract commands for tags
        let mut tags = Vec::new();
        if let Some(commands) = manifest["commands"].as_array() {
            for cmd in commands {
                if let Some(cmd_name) = cmd["name"].as_str() {
                    tags.push(format!("cmd:{}", cmd_name));
                }
            }
        }

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Moltbot,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author,
                tags,
                ..Default::default()
            },
            format: SkillFormat::MoltbotSkill,
        };

        Ok(Some(skill))
    }

    async fn parse_skill_md(&self, path: PathBuf, skill_md_path: PathBuf) -> Result<Option<Skill>> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        // Read first line as description if available
        let description = std::fs::read_to_string(&skill_md_path)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "Moltbot skill".to_string());

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Moltbot,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags: vec!["skill-md".to_string()],
                ..Default::default()
            },
            format: SkillFormat::MoltbotSkill,
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
            description: "Moltbot skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Moltbot,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata::default(),
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
