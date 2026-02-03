use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CloudflareProvider, CodexProvider, CursorProvider,
    KiloProvider, LocalProvider, MoltbotProvider, OpenClawProvider, RooProvider, VercelProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};

pub async fn update_skill(name: Option<&str>) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    // Add all providers
    reader.add_provider(ClaudeProvider);
    reader.add_provider(ClineProvider);
    reader.add_provider(CursorProvider);
    reader.add_provider(RooProvider);
    reader.add_provider(OpenClawProvider);
    reader.add_provider(MoltbotProvider);
    reader.add_provider(CodexProvider);
    reader.add_provider(KiloProvider);
    reader.add_provider(VercelProvider);
    reader.add_provider(CloudflareProvider);
    reader.add_provider(LocalProvider);

    let skills = reader.list_all_skills().await?;

    if let Some(name) = name {
        // Update specific skill
        let name_lower = name.to_lowercase();
        let matching: Vec<_> = skills
            .iter()
            .filter(|s| {
                s.name.to_lowercase() == name_lower || s.id.to_lowercase() == name_lower
            })
            .collect();

        if matching.is_empty() {
            println!("Skill '{}' not found.", name);
            return Ok(());
        }

        for skill in matching {
            update_single_skill(skill).await?;
        }
    } else {
        // Update all skills
        println!("Checking for updates for {} skill(s)...", skills.len());
        for skill in &skills {
            update_single_skill(skill).await?;
        }
    }

    Ok(())
}

async fn update_single_skill(skill: &alltheskills::Skill) -> Result<(), anyhow::Error> {
    use alltheskills::types::SkillSource;

    match &skill.source {
        SkillSource::GitHub {
            owner,
            repo,
            subdir: _,
            branch: _,
        } => {
            println!(
                "Updating {} (GitHub: {}/{})...",
                skill.name, owner, repo
            );
            // TODO: Implement git pull/update logic
            println!("  Note: GitHub skill updates not yet implemented");
        }
        SkillSource::Local { path: _ } => {
            // Local skills can't be automatically updated
            println!("Skipping {} (local skill)", skill.name);
        }
        SkillSource::Remote { url, .. } => {
            println!("Updating {} from {}...", skill.name, url);
            // TODO: Implement remote update logic
            println!("  Note: Remote skill updates not yet implemented");
        }
    }

    Ok(())
}
