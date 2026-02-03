use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CloudflareProvider, CodexProvider, CursorProvider,
    KiloProvider, LocalProvider, MoltbotProvider, OpenClawProvider, RooProvider, VercelProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};

pub async fn remove_skill(name: &str, force: bool) -> Result<(), anyhow::Error> {
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

    let name_lower = name.to_lowercase();

    // Find the skill
    let skills = reader
        .search_skills(|s| s.name.to_lowercase() == name_lower || s.id.to_lowercase() == name_lower)
        .await?;

    if skills.is_empty() {
        println!("Skill '{}' not found.", name);
        println!("Try 'alltheskills list' to see available skills.");
        return Ok(());
    }

    let skill = &skills[0];

    // Confirm removal unless --force
    if !force {
        print!(
            "Are you sure you want to remove '{}' from {:?}? [y/N] ",
            skill.name, skill.path
        );
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Removal cancelled.");
            return Ok(());
        }
    }

    // Remove the skill directory
    let path = &skill.path;
    if path.exists() {
        std::fs::remove_dir_all(path)?;
        println!("Successfully removed skill '{}' from {:?}", skill.name, skill.path);
    } else {
        println!("Skill path does not exist: {:?}", path);
    }

    Ok(())
}

pub async fn remove_source(name: &str) -> Result<(), anyhow::Error> {
    use crate::config;

    let mut cfg = config::load_config()?;

    let initial_len = cfg.sources.len();
    cfg.sources.retain(|s| s.name != name);

    if cfg.sources.len() < initial_len {
        config::save_config(&cfg)?;
        println!("Removed source '{}' from configuration", name);
    } else {
        println!("Source '{}' not found in configuration", name);
    }

    Ok(())
}
