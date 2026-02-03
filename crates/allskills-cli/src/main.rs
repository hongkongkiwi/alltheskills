use clap::{Parser, Subcommand};

mod skill_exporter;
mod commands;

#[derive(Parser)]
#[command(name = "allskills")]
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
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Commands::List { scope } => {
            let scope = scope.map(|s| match s.to_lowercase().as_str() {
                "global" => allskills::SkillScope::Global,
                "user" => allskills::SkillScope::User,
                "project" => allskills::SkillScope::Project,
                _ => allskills::SkillScope::User,
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
    }

    Ok(())
}
