use alltheskills::providers::{
    ClaudeProvider, ClineProvider, CloudflareProvider, CodexProvider, CursorProvider,
    KiloProvider, LocalProvider, MoltbotProvider, OpenClawProvider, RooProvider, VercelProvider,
};
use alltheskills::{AllSkillsConfig, SkillReader};
use std::path::PathBuf;

pub async fn validate_skill(path: Option<&str>) -> Result<(), anyhow::Error> {
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

    if let Some(path) = path {
        // Validate specific skill directory
        validate_single_skill(PathBuf::from(path)).await?;
    } else {
        // Validate all installed skills
        let skills = reader.list_all_skills().await?;
        println!("Validating {} skill(s)...\n", skills.len());

        let mut valid_count = 0;
        let mut invalid_count = 0;

        for skill in &skills {
            match validate_skill_structure(&skill.path, &format!("{:?}", skill.source_type)).await {
                Ok(()) => {
                    println!("✅ {} - Valid", skill.name);
                    valid_count += 1;
                }
                Err(e) => {
                    println!("❌ {} - Invalid: {}", skill.name, e);
                    invalid_count += 1;
                }
            }
        }

        println!(
            "\nValidation complete: {} valid, {} invalid",
            valid_count, invalid_count
        );
    }

    Ok(())
}

async fn validate_single_skill(path: PathBuf) -> Result<(), anyhow::Error> {
    println!("Validating skill at: {}\n", path.display());

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    if !path.is_dir() {
        anyhow::bail!("Path is not a directory: {}", path.display());
    }

    // Check for required files
    let mut has_manifest = false;
    let mut has_readme = false;

    // Check for various manifest files
    let manifest_files = [
        "claude.json",
        "cline.json",
        "cursor.json",
        "roo.json",
        ".roomodes",
        "manifest.json",
        "skill.json",
        "codex.json",
        "kilo.yaml",
        "kilo.yml",
        "wrangler.toml",
    ];

    for file in &manifest_files {
        if path.join(file).exists() {
            has_manifest = true;
            println!("✅ Found manifest: {}", file);

            // Validate JSON files
            if file.ends_with(".json") && !file.starts_with(".") {
                let content = std::fs::read_to_string(path.join(file))?;
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(_) => println!("   ✅ Valid JSON"),
                    Err(e) => println!("   ❌ Invalid JSON: {}", e),
                }
            }
        }
    }

    // Check for README
    if path.join("README.md").exists() {
        has_readme = true;
        println!("✅ Found README.md");
    }

    // Check for SKILL.md (Moltbot format)
    if path.join("SKILL.md").exists() {
        println!("✅ Found SKILL.md");
    }

    // Check for .cursorrules (Cursor format)
    if path.join(".cursorrules").exists() {
        has_manifest = true;
        println!("✅ Found .cursorrules");
    }

    println!();

    if !has_manifest {
        println!("⚠️  Warning: No recognized manifest file found");
    }

    if !has_readme {
        println!("⚠️  Warning: No README.md found");
    }

    if has_manifest {
        println!("\n✅ Skill structure appears valid");
        Ok(())
    } else {
        anyhow::bail!("Skill is missing required manifest file");
    }
}

async fn validate_skill_structure(path: &PathBuf, source_type: &str) -> Result<(), anyhow::Error> {
    if !path.exists() {
        anyhow::bail!("Path does not exist");
    }

    // Check for appropriate files based on source type
    let required_files: &[&str] = match source_type.to_lowercase().as_str() {
        "claude" => &["claude.json", "skill.md", "README.md"],
        "cline" => &["cline.json", "custom-instructions.md", "README.md"],
        "cursor" => &[".cursorrules", "cursor.json", "README.md"],
        "roocode" => &["roo.json", ".roomodes", "README.md"],
        "moltbot" => &["manifest.json", "SKILL.md", "README.md"],
        "openclaw" => &["skill.json", "README.md"],
        "openaicodex" => &["codex.json", "instructions.md", "README.md"],
        "kilocode" => &["kilo.yaml", "kilo.yml", "instructions.md", "README.md"],
        _ => &["README.md"],
    };

    let has_required = required_files.iter().any(|file| path.join(file).exists());

    if !has_required {
        anyhow::bail!(
            "Missing required files. Expected one of: {:?}",
            required_files
        );
    }

    Ok(())
}
