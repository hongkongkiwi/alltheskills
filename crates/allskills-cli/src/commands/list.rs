use allskills::{SkillReader, AllSkillsConfig};

pub async fn list_skills(scope: Option<allskills::SkillScope>) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    // Add all providers
    reader.add_provider(allskills::providers::claude::ClaudeProvider);
    reader.add_provider(allskills::providers::local::LocalProvider);
    reader.add_provider(allskills::providers::openclaw::OpenClawProvider);

    let skills = reader.list_all_skills().await?;

    if skills.is_empty() {
        println!("No skills found.");
        println!("Try adding a skill source with: allskills add-source --path <path> --source-type <type>");
        return Ok(());
    }

    println!("Found {} skill(s):\n", skills.len());
    for skill in skills {
        println!(
            "[{:?}] {} - {}",
            skill.source_type,
            skill.name,
            skill.description
        );
        if let Some(version) = &skill.version {
            println!("       Version: {}", version);
        }
        println!("       Path: {}", skill.path.display());
        println!();
    }

    Ok(())
}
