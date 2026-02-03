use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CursorProvider, LocalProvider, MoltbotProvider,
    OpenClawProvider, RooProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};

pub async fn list_skills(_scope: Option<alltheskills::SkillScope>) -> Result<(), anyhow::Error> {
    let config = AllSkillsConfig::default();
    let mut reader = SkillReader::new(config);

    // Add all providers
    reader.add_provider(ClaudeProvider);
    reader.add_provider(ClineProvider);
    reader.add_provider(CursorProvider);
    reader.add_provider(RooProvider);
    reader.add_provider(OpenClawProvider);
    reader.add_provider(MoltbotProvider);
    reader.add_provider(LocalProvider);

    let skills = reader.list_all_skills().await?;

    if skills.is_empty() {
        println!("No skills found.");
        println!();
        println!("Try one of the following:");
        println!("  - Install a skill: alltheskills install <source>");
        println!("  - Add a source: alltheskills add-source <name> <path> --source-type <type>");
        println!();
        println!("Supported source types: claude, cline, cursor, roo, openclaw, moltbot, local");
        return Ok(());
    }

    println!("Found {} skill(s):\n", skills.len());
    for skill in skills {
        println!(
            "[{:?}] {} - {}",
            skill.source_type, skill.name, skill.description
        );
        if let Some(version) = &skill.version {
            println!("       Version: {}", version);
        }
        if !skill.metadata.tags.is_empty() {
            println!("       Tags: {}", skill.metadata.tags.join(", "));
        }
        println!("       Path: {}", skill.path.display());
        println!();
    }

    Ok(())
}
