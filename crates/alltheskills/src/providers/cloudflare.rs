use crate::types::{Skill, SkillFormat, SkillMetadata, SkillSource, SourceConfig, SourceType};
use crate::{Error, Result};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct CloudflareProvider;

#[async_trait]
impl crate::providers::SkillProvider for CloudflareProvider {
    fn name(&self) -> &'static str {
        "Cloudflare Workers AI Skills"
    }

    fn source_type(&self) -> SourceType {
        SourceType::Custom("cloudflare".to_string())
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::Local { path } if
            path.to_string_lossy().contains("cloudflare") ||
            path.to_string_lossy().contains("workers"))
    }

    async fn list_skills(&self, config: &SourceConfig) -> Result<Vec<Skill>> {
        // Check if this source type matches our provider
        let is_our_type = match &config.source_type {
            SourceType::Custom(name) => name == "cloudflare",
            _ => false,
        };

        if !is_our_type {
            return Ok(vec![]);
        }

        // Get path from KnownSources or return empty for now
        let path = match crate::providers::KnownSources::cloudflare_skills_dir() {
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

    async fn install(&self, _source: SkillSource, _target: PathBuf) -> Result<Skill> {
        // Cloudflare workers can be installed via wrangler or direct deploy
        Err(Error::Install {
            reason: "Install via wrangler CLI: npx wrangler deploy".to_string(),
        })
    }
}

impl CloudflareProvider {
    async fn parse_skill_dir(&self, path: PathBuf) -> Result<Option<Skill>> {
        let worker_path = path.join("worker.js");
        let ts_path = path.join("worker.ts");
        let config_path = path.join("wrangler.toml");

        if worker_path.exists() || ts_path.exists() {
            self.parse_worker(path, worker_path.exists()).await
        } else if config_path.exists() {
            self.parse_wrangler_config(path, config_path).await
        } else {
            Ok(None)
        }
    }

    async fn parse_worker(&self, path: PathBuf, is_js: bool) -> Result<Option<Skill>> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        // Try to read wrangler.toml for metadata
        let config_path = path.join("wrangler.toml");
        let description = if config_path.exists() {
            std::fs::read_to_string(&config_path)
                .ok()
                .and_then(|c| {
                    c.lines()
                        .find(|l| l.starts_with("description"))
                        .and_then(|l| l.split('=').nth(1))
                        .map(|d| d.trim().trim_matches('"').to_string())
                })
                .unwrap_or_else(|| "Cloudflare Workers AI skill".to_string())
        } else {
            "Cloudflare Workers AI skill".to_string()
        };

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.replace("-", " ").replace("_", " "),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("cloudflare".to_string()),
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                homepage: Some("https://developers.cloudflare.com/workers/".to_string()),
                ..Default::default()
            },
            format: if is_js {
                SkillFormat::GenericJson
            } else {
                SkillFormat::Unknown
            },
        };

        Ok(Some(skill))
    }

    async fn parse_wrangler_config(
        &self,
        path: PathBuf,
        config_path: PathBuf,
    ) -> Result<Option<Skill>> {
        let content = std::fs::read_to_string(&config_path)?;
        let config: toml::Value = content.parse().map_err(|e| Error::Parse {
            message: format!("Failed to parse wrangler.toml: {}", e),
        })?;

        let name = config
            .get("name")
            .and_then(|v| v.as_str())
            .or(path.file_name().and_then(|n| n.to_str()))
            .unwrap_or("cloudflare-worker")
            .to_string();

        let description = config
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("Cloudflare Workers AI skill")
            .to_string();

        let skill = Skill {
            id: name.to_lowercase().replace(" ", "-"),
            name: name.replace("-", " ").replace("_", " "),
            description,
            version: None,
            source: SkillSource::Local { path: path.clone() },
            source_type: SourceType::Custom("cloudflare".to_string()),
            path,
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                homepage: Some("https://developers.cloudflare.com/workers/".to_string()),
                ..Default::default()
            },
            format: SkillFormat::Unknown,
        };

        Ok(Some(skill))
    }
}
