/// ## Tokens List
///
///  The `Tokens list` function displays information about the project tokens without hash value
///
mod list;
use crate::tokens::list::{list, ProjectsTokenListArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct TokenCli {
    #[clap(subcommand)]
    commands: ProjectsTokenCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "View a project token list")]
pub enum ProjectsTokenCommands {
    #[clap(about = "List all project token")]
    List(ProjectsTokenListArgs),
}

pub fn match_command(input: &TokenCli) {
    match &input.commands {
        ProjectsTokenCommands::List(args) => list(&args),
    }
}
