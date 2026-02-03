use alltheskills::SkillProvider;
use alltheskills::providers::github::GitHubProvider;
use alltheskills::providers::local::LocalProvider;
use alltheskills::types::SkillSource;
use std::path::PathBuf;

pub async fn install_skill(source: &str, target: Option<&str>) -> Result<(), anyhow::Error> {
    let target_path = target
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".alltheskills"));

    let skill_source = if source.starts_with("https://github.com/") {
        // Parse GitHub URL
        let url = source.trim_start_matches("https://github.com/");
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() >= 2 {
            let owner = parts[0].to_string();
            let repo = parts[1].to_string();
            let subdir = if parts.len() > 2 {
                Some(parts[2..].join("/"))
            } else {
                None
            };

            println!("Installing skill from GitHub: {}/{}", owner, repo);

            let provider = GitHubProvider;
            let source = SkillSource::GitHub {
                owner,
                repo,
                subdir,
                branch: None,
            };

            provider.install(source, target_path).await?
        } else {
            anyhow::bail!("Invalid GitHub URL: {}", source);
        }
    } else {
        // Local path
        println!("Installing skill from local path: {}", source);

        let provider = LocalProvider;
        let source = SkillSource::Local {
            path: PathBuf::from(source),
        };

        provider.install(source, target_path).await?
    };

    println!("Successfully installed skill: {}", skill_source.name);

    Ok(())
}
