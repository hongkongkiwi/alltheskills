use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CursorProvider, LocalProvider, MoltbotProvider,
    OpenClawProvider, RooProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};

pub async fn search_skills(query: &str) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    reader.add_provider(ClaudeProvider);
    reader.add_provider(ClineProvider);
    reader.add_provider(CursorProvider);
    reader.add_provider(RooProvider);
    reader.add_provider(OpenClawProvider);
    reader.add_provider(MoltbotProvider);
    reader.add_provider(LocalProvider);

    let query_lower = query.to_lowercase();

    let skills = reader
        .search_skills(|s| {
            s.name.to_lowercase().contains(&query_lower)
                || s.description.to_lowercase().contains(&query_lower)
                || s.metadata
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&query_lower))
        })
        .await?;

    if skills.is_empty() {
        println!("No skills found matching '{}'.", query);
    } else {
        println!("Found {} skill(s) matching '{}':\n", skills.len(), query);
        for skill in skills {
            println!("[{:?}] {}", skill.source_type, skill.name);
            println!("  {}", skill.description);
            if !skill.metadata.tags.is_empty() {
                println!("  Tags: {}", skill.metadata.tags.join(", "));
            }
            println!();
        }
    }

    Ok(())
}
