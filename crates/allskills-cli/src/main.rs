use clap::{Parser, Subcommand};

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
}

fn main() {
    let args = Args::parse();
    println!("AllSkills CLI");
}
