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
    List,
    Install,
    Search,
    Info,
    ExportAsSkill {
        #[arg(short, long)]
        output_dir: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Commands::List => {
            println!("List command");
        }
        Commands::Install => {
            println!("Install command");
        }
        Commands::Search => {
            println!("Search command");
        }
        Commands::Info => {
            println!("Info command");
        }
        Commands::ExportAsSkill { output_dir } => {
            commands::export_skill::export_as_skill(output_dir).await?;
        }
    }

    Ok(())
}
