use clap::{Parser, Subcommand};

mod commands;
mod config;
mod skill_exporter;

#[derive(Parser)]
#[command(name = "alltheskills")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all installed skills
    List {
        /// Filter by scope (global, user, project)
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Install a skill from a source
    Install {
        /// Source URL or path (GitHub URL or local directory)
        source: String,
        /// Target directory for installation
        #[arg(short, long)]
        target: Option<String>,
    },
    /// Search for skills by name, description, or tags
    Search {
        /// Search query
        query: String,
    },
    /// Show detailed information about a specific skill
    Info {
        /// Skill name or ID
        name: String,
    },
    /// Export a project as a skill
    ExportAsSkill {
        #[arg(short, long)]
        output_dir: Option<String>,
    },
    /// Add a new source to the configuration
    AddSource {
        /// Name for the source
        name: String,
        /// Path or URL to the source
        path: String,
        /// Source type (claude, cline, openclaw, roo, github, local, etc.)
        #[arg(short, long)]
        source_type: String,
        /// Scope for the source (global, user, project)
        #[arg(long, default_value = "user")]
        scope: String,
    },
    /// Remove a source from the configuration
    RemoveSource {
        /// Name of the source to remove
        name: String,
    },
    /// Show the current configuration
    Config {
        /// Show the config file path
        #[arg(short, long)]
        path: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Commands::List { scope } => {
            let scope = scope.map(|s| match s.to_lowercase().as_str() {
                "global" => alltheskills::SkillScope::Global,
                "user" => alltheskills::SkillScope::User,
                "project" => alltheskills::SkillScope::Project,
                _ => alltheskills::SkillScope::User,
            });
            commands::list_skills(scope).await?;
        }
        Commands::Install { source, target } => {
            commands::install_skill(&source, target.as_deref()).await?;
        }
        Commands::Search { query } => {
            commands::search_skills(&query).await?;
        }
        Commands::Info { name } => {
            commands::info_skill(&name).await?;
        }
        Commands::ExportAsSkill { output_dir } => {
            commands::export_as_skill(output_dir).await?;
        }
        Commands::AddSource {
            name,
            path,
            source_type,
            scope,
        } => {
            let scope = match scope.to_lowercase().as_str() {
                "global" => alltheskills::SkillScope::Global,
                "user" => alltheskills::SkillScope::User,
                "project" => alltheskills::SkillScope::Project,
                _ => alltheskills::SkillScope::User,
            };
            let mut config = config::load_config()?;
            config::add_source(&mut config, &name, &path, &source_type, scope);
            config::save_config(&config)?;
            println!("Added source '{}' to configuration", name);
        }
        Commands::RemoveSource { name } => {
            let mut config = config::load_config()?;
            if config::remove_source(&mut config, &name) {
                config::save_config(&config)?;
                println!("Removed source '{}' from configuration", name);
            } else {
                println!("Source '{}' not found in configuration", name);
            }
        }
        Commands::Config { path } => {
            if path {
                println!("Config path: {}", config::get_config_path().display());
            } else {
                let config = config::load_config()?;
                println!("Current configuration:");
                println!("  Version: {}", config.version);
                println!("  Default scope: {:?}", config.default_scope);
                println!("  Install dir: {}", config.install_dir.display());
                println!("  Cache dir: {}", config.cache_dir.display());
                println!("  Sources:");
                for source in &config.sources {
                    println!(
                        "    - {} (type: {:?}, scope: {:?}, enabled: {})",
                        source.name, source.source_type, source.scope, source.enabled
                    );
                }
            }
        }
    }

    Ok(())
}
