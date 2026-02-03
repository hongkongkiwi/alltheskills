use std::path::PathBuf;
use crate::skill_exporter::{generate_allskills_skill_readme, generate_claude_json};

pub async fn export_as_skill(output_dir: Option<String>) -> Result<(), anyhow::Error> {
    let output = output_dir.unwrap_or_else(|| ".allskills/skill".to_string());
    let output_path = PathBuf::from(&output);

    // Create skill directory
    std::fs::create_dir_all(&output_path)?;

    // Write README.md
    let readme = generate_allskills_skill_readme();
    std::fs::write(output_path.join("README.md"), readme)?;

    // Write claude.json
    let skill_json = generate_claude_json();
    std::fs::write(output_path.join("claude.json"), skill_json)?;

    println!("Skill exported to: {}", output);
    println!("To install in Claude Code, run:");
    println!("  cp -r {} ~/.claude/skills/allskills-manager", output);

    Ok(())
}
