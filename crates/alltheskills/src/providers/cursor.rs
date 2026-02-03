use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::utils::copy_skill_dir;
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Provider for Cursor editor skills
///
/// Cursor stores custom instructions in `.cursorrules` files at:
/// - `~/.cursor/rules/` - Global user rules
/// - Project root `.cursorrules` - Project-specific rules
///
/// # Skill Structure
/// Cursor skills are typically defined via:
/// - `.cursorrules` - Main rules file (markdown format)
/// - `cursor.json` - Optional metadata configuration
///
/// Reference: <https://cursor.sh/>
pub struct CursorProvider;

#[async_trait]
impl crate::providers::SkillProvider for CursorProvider {
    fn name(&self) -> &'static str {
        "Cursor Rules"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Cursor
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if
            path.to_string_lossy().contains("cursor") ||
            path.extension().is_some_and(|e| e == "cursorrules")
        )
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        let path = match &config.source_type {
            SourceType::Cursor => dirs::home_dir()
                .map(|h| h.join(".cursor/rules"))
                .unwrap_or_else(|| PathBuf::from(".cursor/rules")),
            _ => return Ok(vec![]),
        };

        let mut skills = Vec::new();

        // List global rules from ~/.cursor/rules/
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(skill) = self.parse_rules_file(entry_path).await? {
                        skills.push(skill);
                    }
                } else if entry_path.is_dir()
                    && let Some(skill) = self.parse_skill_dir(entry_path).await?
                {
                    skills.push(skill);
                }
            }
        }

        // Also check for project-level .cursorrules in common locations
        if let Ok(cwd) = std::env::current_dir() {
            let project_rules = cwd.join(".cursorrules");
            if project_rules.exists() {
                if let Some(skill) = self.parse_rules_file(project_rules).await? {
                    skills.push(skill);
                }
            }

            // Check .cursor/ directory in project
            let cursor_dir = cwd.join(".cursor");
            if cursor_dir.exists() {
                let rules_dir = cursor_dir.join("rules");
                if rules_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(&rules_dir) {
                        for entry in entries.flatten() {
                            let entry_path = entry.path();
                            if entry_path.is_file() {
                                if let Some(skill) = self.parse_rules_file(entry_path).await? {
                                    skills.push(skill);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(skills)
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String> {
        // Try to read the rules file content
        std::fs::read_to_string(&skill.path).map_err(Error::from)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill> {
        let source_path = match &source {
            SkillSource::Local { path } => path.clone(),
            _ => {
                return Err(Error::Install {
                    reason: "Cursor provider only supports local installation".to_string(),
                })
            }
        };

        std::fs::create_dir_all(&target)?;
        copy_skill_dir(&source_path, &target)?;

        self.parse_skill_dir(target.clone())
            .await?
            .ok_or_else(|| Error::Install {
                reason: "Failed to parse installed Cursor skill".to_string(),
            })
    }
}

impl CursorProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        let rules_path = path.join(".cursorrules");
        let json_path = path.join("cursor.json");
        let readme_path = path.join("README.md");

        if rules_path.exists() {
            self.parse_rules_file(rules_path).await
        } else if json_path.exists() {
            self.parse_cursor_json(path, json_path).await
        } else if readme_path.exists() {
            self.parse_markdown(path, readme_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_rules_file(&self, path: PathBuf) -> Result<Option<Skill>> {
        let name = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("cursor-rules")
            .to_string();

        // Read first non-empty line as description
        let description = std::fs::read_to_string(&path)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| !line.trim().is_empty())
                    .map(|line| {
                        let desc = line.trim();
                        if desc.len() > 80 {
                            format!("{}...", &desc[..77])
                        } else {
                            desc.to_string()
                        }
                    })
            })
            .unwrap_or_else(|| "Cursor custom rules".to_string());

        // Determine if it's project-level or global
        let is_project_level = path.to_string_lossy().contains("/.cursorrules")
            || path.to_string_lossy().contains("/projects/");

        let mut tags = vec!["cursor".to_string()];
        if is_project_level {
            tags.push("project-level".to_string());
        } else {
            tags.push("global".to_string());
        }

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-").replace(".", "-"),
            name: name.clone(),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Cursor,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags,
                ..Default::default()
            },
            format: SkillFormat::CursorRules,
        };

        Ok(Some(skill))
    }

    async fn parse_cursor_json(&self, path: PathBuf, json_path: PathBuf) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&json_path)?;
        let config: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| Error::Parse {
                message: format!("Failed to parse cursor.json: {}", e),
            })?;

        let name = config["name"].as_str().unwrap_or_default().to_string();
        let description = config["description"]
            .as_str()
            .unwrap_or("Cursor configuration")
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name,
            description,
            version: config["version"].as_str().map(|s| s.to_string()),
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Cursor,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                author: config["author"].as_str().map(|s| s.to_string()),
                tags: vec!["cursor".to_string(), "json-config".to_string()],
                ..Default::default()
            },
            format: SkillFormat::CursorRules,
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
            description: "Cursor skill (markdown format)".to_string(),
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Cursor,
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                tags: vec!["cursor".to_string()],
                ..Default::default()
            },
            format: SkillFormat::GenericMarkdown,
        };

        Ok(Some(skill))
    }
}
