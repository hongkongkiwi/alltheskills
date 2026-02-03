use allskills::{SkillReader, AllSkillsConfig};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    reader.add_provider(allskills::providers::ClaudeProvider);
    reader.add_provider(allskills::providers::LocalProvider);
    reader.add_provider(allskills::providers::OpenClawProvider);

    // Search by name pattern
    let results = reader
        .search_skills(|s| s.name.contains("database"))
        .await?;

    println!("Found {} database-related skills:", results.len());
    for skill in results {
        println!("  - {} ({:?})", skill.name, skill.source_type);
    }

    Ok(())
}
