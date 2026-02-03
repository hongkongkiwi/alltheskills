use std::path::PathBuf;
use alltheskills::{AllSkillsConfig, SkillScope, SourceType};
use alltheskills::types::SourceConfig;

const CONFIG_FILENAME: &str = "alltheskills.toml";

/// Get the platform-appropriate config directory
pub fn get_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join("Library/Application Support")
        } else {
            PathBuf::from("/Library/Application Support")
        }
    }
    #[cfg(windows)]
    {
        if let Ok(val) = std::env::var("APPDATA") {
            PathBuf::from(val)
        } else {
            PathBuf::from(std::env::temp_dir())
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Ok(val) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(val)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            PathBuf::from(".config")
        }
    }
}

/// Get the config file path
pub fn get_config_path() -> PathBuf {
    get_config_dir().join("alltheskills").join(CONFIG_FILENAME)
}

/// Load configuration from the config file
pub fn load_config() -> Result<AllSkillsConfig, anyhow::Error> {
    let config_path = get_config_path();

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: AllSkillsConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        // Create default config
        let config = AllSkillsConfig::default();
        save_config(&config)?;
        Ok(config)
    }
}

/// Save configuration to the config file
pub fn save_config(config: &AllSkillsConfig) -> Result<(), anyhow::Error> {
    let config_dir = get_config_dir().join("alltheskills");
    std::fs::create_dir_all(&config_dir)?;

    let content = toml::to_string_pretty(config)?;
    std::fs::write(config_dir.join(CONFIG_FILENAME), content)?;

    Ok(())
}

/// Add a new source to the configuration
pub fn add_source(
    config: &mut AllSkillsConfig,
    name: &str,
    path: &str,
    source_type: &str,
    scope: SkillScope,
) {
    let source_config = SourceConfig {
        name: name.to_string(),
        source_type: match source_type.to_lowercase().as_str() {
            "claude" => SourceType::Claude,
            "cline" => SourceType::Cline,
            "openclaw" => SourceType::Custom("openclaw".to_string()),
            "roo" | "roocode" => SourceType::RooCode,
            "codex" | "openai" => SourceType::OpenAICodex,
            "kilo" => SourceType::KiloCode,
            "github" => SourceType::GitHub,
            "local" => SourceType::Local,
            _ => SourceType::Custom(source_type.to_string()),
        },
        enabled: true,
        scope,
        priority: config.sources.len() as i32,
    };
    config.sources.push(source_config);
}

/// Remove a source from the configuration
pub fn remove_source(config: &mut AllSkillsConfig, name: &str) -> bool {
    let initial_len = config.sources.len();
    config.sources.retain(|s| s.name != name);
    config.sources.len() < initial_len
}

/// Get known skill directories based on platform
pub fn get_known_skill_directories() -> Vec<(String, PathBuf)> {
    let mut dirs = Vec::new();

    // Check Claude
    if let Some(path) = alltheskills::KnownSources::claude_skills_dir() {
        dirs.push(("Claude".to_string(), path));
    }

    // Check Cline
    if let Some(path) = alltheskills::KnownSources::cline_skills_dir() {
        dirs.push(("Cline".to_string(), path));
    }

    // Check OpenClaw
    if let Some(path) = alltheskills::KnownSources::openclaw_skills_dir() {
        dirs.push(("OpenClaw".to_string(), path));
    }

    // Check Roo Code
    if let Some(path) = alltheskills::KnownSources::roo_skills_dir() {
        dirs.push(("Roo Code".to_string(), path));
    }

    dirs
}
