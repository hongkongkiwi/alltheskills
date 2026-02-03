use allskills::{SkillReader, AllSkillsConfig};

pub async fn info_skill(name: &str) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    reader.add_provider(allskills::providers::claude::ClaudeProvider);
    reader.add_provider(allskills::providers::local::LocalProvider);
    reader.add_provider(allskills::providers::openclaw::OpenClawProvider);

    let name_lower = name.to_lowercase();

    let skills = reader.search_skills(|s| {
        s.name.to_lowercase() == name_lower
            || s.id.to_lowercase() == name_lower
    }).await?;

    if skills.is_empty() {
        println!("Skill '{}' not found.", name);
        println!("Try 'allskills list' to see available skills.");
    } else {
        let skill = &skills[0];
        println!("Skill: {}", skill.name);
        println!("ID: {}", skill.id);
        println!("Description: {}", skill.description);
        println!("Format: {:?}", skill.format);
        println!("Source: {:?}", skill.source_type);
        println!("Path: {}", skill.path.display());
        println!("Installed: {}", skill.installed_at);

        if let Some(version) = &skill.version {
            println!("Version: {}", version);
        }
        if let Some(author) = &skill.metadata.author {
            println!("Author: {}", author);
        }
        if !skill.metadata.tags.is_empty() {
            println!("Tags: {}", skill.metadata.tags.join(", "));
        }
    }

    Ok(())
}
