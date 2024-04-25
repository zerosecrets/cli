/// ## Tokens List
///
///  The `Tokens list` function displays information about the project tokens without hash value
///
mod list;
/// ## Create Tokens
///
///  The `Create list` function create new token for project
///
mod create;
use crate::tokens::list::{list, TokenListArgs};
use crate::tokens::create::{create, TokenCreateArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct TokenCli {
    #[clap(subcommand)]
    commands: TokenCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "View a project token list")]
pub enum TokenCommands {
    #[clap(about = "List all project token")]
    List(TokenListArgs),
    #[clap(about = "Create project token")]
    Create(TokenCreateArgs),
}

pub fn match_command(input: &TokenCli) {
    match &input.commands {
        TokenCommands::List(args) => list(&args),
        TokenCommands::Create(args) => create(&args),
    }
}
