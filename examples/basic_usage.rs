use allskills::{SkillReader, AllSkillsConfig, SkillScope};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    let config = AllSkillsConfig::default();

    // Create skill reader with all providers
    let mut reader = SkillReader::new(config);

    // Add providers
    reader.add_provider(allskills::providers::ClaudeProvider);
    reader.add_provider(allskills::providers::LocalProvider);
    reader.add_provider(allskills::providers::OpenClawProvider);

    // List all skills from all sources
    println!("Listing all skills...");
    let skills = reader.list_all_skills().await?;

    println!("Found {} skill(s):\n", skills.len());
    for skill in skills {
        println!(
            "[{:?}] {} - {}",
            skill.source_type,
            skill.name,
            skill.description
        );
    }

    // Search for skills
    println!("\nSearching for 'git' skills...");
    let git_skills = reader
        .search_skills(|s| {
            s.name.to_lowercase().contains("git")
                || s.description.to_lowercase().contains("git")
                || s.metadata.tags.iter().any(|t| t.to_lowercase().contains("git"))
        })
        .await?;

    println!("Found {} skill(s) matching 'git':", git_skills.len());
    for skill in git_skills {
        println!("  - {}", skill.name);
    }

    Ok(())
}
