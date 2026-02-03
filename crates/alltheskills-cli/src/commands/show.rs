use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CloudflareProvider, CodexProvider, CursorProvider,
    KiloProvider, LocalProvider, MoltbotProvider, OpenClawProvider, RooProvider, VercelProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};


/// Show the content of a skill
pub async fn show_skill(name: &str, raw: bool) -> Result<(), anyhow::Error> {
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

    if raw {
        // Just output the raw content without formatting
        let content = get_skill_content(skill).await?;
        println!("{}", content);
    } else {
        // Pretty formatted output
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║  {:<58} ║", skill.name);
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  Source: {:<52} ║", format!("{:?}", skill.source_type));
        println!("║  Path: {:<54} ║", skill.path.display().to_string());
        if let Some(version) = &skill.version {
            println!("║  Version: {:<51} ║", version);
        }
        if let Some(author) = &skill.metadata.author {
            println!("║  Author: {:<52} ║", author);
        }
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();

        // Show content
        let content = get_skill_content(skill).await?;
        println!("{}", content);
    }

    Ok(())
}

async fn get_skill_content(skill: &alltheskills::Skill) -> Result<String, anyhow::Error> {

    // Try to read the main content file based on source type and available files
    let path = &skill.path;

    // Priority order for content files
    let content_files: Vec<&str> = match skill.source_type {
        alltheskills::types::SourceType::Claude => {
            vec!["skill.md", "README.md", "claude.json"]
        }
        alltheskills::types::SourceType::Cline => {
            vec!["custom-instructions.md", "README.md", "cline.json"]
        }
        alltheskills::types::SourceType::Cursor => {
            vec![".cursorrules", "README.md", "cursor.json"]
        }
        alltheskills::types::SourceType::RooCode => {
            vec!["README.md", ".roomodes", "roo.json"]
        }
        alltheskills::types::SourceType::OpenAICodex => {
            vec!["instructions.md", "README.md", "codex.json"]
        }
        alltheskills::types::SourceType::KiloCode => {
            vec!["instructions.md", "README.md", "kilo.yaml", "kilo.yml"]
        }
        alltheskills::types::SourceType::Moltbot => {
            vec!["SKILL.md", "README.md", "manifest.json"]
        }
        alltheskills::types::SourceType::OpenClaw => {
            vec!["README.md", "skill.json"]
        }
        alltheskills::types::SourceType::GitHub => {
            vec!["README.md", "skill.md", "skill.json"]
        }
        _ => {
            vec!["README.md", "skill.md", "skill.json"]
        }
    };

    for file in content_files {
        let file_path = path.join(file);
        if file_path.exists() {
            return std::fs::read_to_string(&file_path)
                .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", file, e));
        }
    }

    // If no content file found, return metadata
    Ok(format!(
        "No content file found for skill: {}\n\nDescription: {}\nFormat: {:?}",
        skill.name, skill.description, skill.format
    ))
}


