mod auth;
mod common;
mod projects;
mod secrets;
mod teams;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="Ottofeller", version="1.0.4", about="CLI for the Zero service", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]

enum Commands {
    #[clap(about = "Authentication and authorization")]
    Auth(auth::AuthCli),
    #[clap(about = "Manage projects")]
    Projects(projects::ProjectsCli),
    #[clap(about = "Manage secrets")]
    Secrets(secrets::SecretsCli),
    #[clap(about = "Manage teams")]
    Teams(teams::TeamsCli),
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Auth(input) => auth::match_command(input),
        Commands::Projects(input) => projects::match_command(input),
        Commands::Secrets(input) => secrets::match_command(input),
        Commands::Teams(input) => teams::match_command(input),
    }
}
