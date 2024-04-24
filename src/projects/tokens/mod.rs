/// ## Projects Tokens List
///
///  The `tokens list` function displays information about the project tokens without hash value
///
mod list;
use crate::projects::tokens::list::{list, ProjectsTokenListArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct ProjectsTokenCommands {
    #[clap(subcommand)]
    commands: ProjectsTokenCommandsEnum,
}

#[derive(Subcommand, Debug)]
#[clap(about = "View a project token list")]
pub enum ProjectsTokenCommandsEnum {
    #[clap(about = "List all project token")]
    List(ProjectsTokenListArgs),
}

pub fn token_commands(input: &ProjectsTokenCommands) {
    match &input.commands {
        ProjectsTokenCommandsEnum::List(args) => list(&args),
    }
}
