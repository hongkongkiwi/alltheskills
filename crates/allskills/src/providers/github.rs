use async_trait::async_trait;
use std::path::PathBuf;
use crate::types::{Skill, SkillFormat, SourceType, SkillSource, SkillMetadata};
use crate::{Result, Error};

pub struct GitHubProvider;

#[async_trait]
impl crate::providers::SkillProvider for GitHubProvider {
    fn name(&self) -> &'static str {
        "GitHub Repository"
    }

    fn source_type(&self) -> SourceType {
        SourceType::GitHub
    }

    fn can_handle(&self, source: &SkillSource) -> bool {
        matches!(source, SkillSource::GitHub { .. })
    }

    async fn list_skills(&self, config: &crate::types::SourceConfig) -> Result<Vec<Skill>> {
        // For GitHub sources, we list skills from the cloned repository
        // This would typically be called after install() has cloned the repo
        Ok(vec![])
    }

    async fn read_skill(&self, skill: &Skill) -> Result<String> {
        let readme_path = skill.path.join("README.md");
        let content = std::fs::read_to_string(&readme_path).map_err(|e| Error::from(e))?;
        Ok(content)
    }

    async fn install(&self, source: SkillSource, target: PathBuf) -> Result<Skill> {
        let SkillSource::GitHub { owner, repo, subdir, branch } = source else {
            return Err(Error::Install { reason: "Invalid source type for GitHub provider".to_string() });
        };

        let repo_name = repo.clone();
        let repo_url = format!("https://github.com/{}/{}", owner, repo);

        // Create parent directories if needed
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Clone the repository
        let repository = git2::Repository::clone(&repo_url, &target)
            .map_err(|e| Error::Install { reason: format!("Failed to clone: {}", e) })?;

        // Checkout specific branch if provided
        if let Some(branch_name) = branch {
            let branch_ref = format!("refs/heads/{}", branch_name);
            match repository.find_branch(&branch_name, git2::BranchType::Local) {
                Ok(_) => {
                    repository.set_head(&branch_ref)?;
                    let _ = repository.checkout_head(None);
                }
                Err(_) => {
                    // Branch not found, stay on default
                }
            }
        }

        // Determine actual skill path
        let skill_path = if let Some(subdir) = subdir {
            target.join(subdir)
        } else {
            target.clone()
        };

        // Parse skill metadata
        let skill = Self::parse_skill_dir(&skill_path, &owner, &repo_name)?;

        Ok(skill)
    }
}

impl GitHubProvider {
    fn parse_skill_dir(path: &PathBuf, owner: &str, repo: &str) -> Result<Skill> {
        let json_path = path.join("claude.json");
        let md_path = path.join("README.md");

        let (name, description, version, format) = if json_path.exists() {
            let content = std::fs::read_to_string(&json_path)?;
            let config: serde_json::Value = serde_json::from_str(&content)?;

            let name = config["name"].as_str().unwrap_or(repo).to_string();
            let description = config["description"].as_str().unwrap_or("").to_string();
            let version = config["version"].as_str().map(|s| s.to_string());
            let format = SkillFormat::ClaudeSkill;

            (name, description, version, format)
        } else {
            let name = repo.to_string();
            let description = format!("Skill from {}/{}", owner, repo);
            let version = None;
            let format = SkillFormat::GenericMarkdown;

            (name, description, version, format)
        };

        let skill = Skill {
            id: format!("{}-{}", owner, repo).to_lowercase().replace(" ", "-"),
            name,
            description,
            version,
            source: SkillSource::GitHub {
                owner: owner.to_string(),
                repo: repo.to_string(),
                subdir: None,
                branch: None,
            },
            source_type: SourceType::GitHub,
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            metadata: SkillMetadata {
                repository: Some(format!("https://github.com/{}/{}", owner, repo)),
                ..Default::default()
            },
            format,
        };

        Ok(skill)
    }
}
