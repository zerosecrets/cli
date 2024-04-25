/// ## Tokens List
///
///  The `Tokens list` function displays information about the project tokens without hash value
///
mod list;
use crate::tokens::list::{list, TokenListArgs};
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
}

pub fn match_command(input: &TokenCli) {
    match &input.commands {
        TokenCommands::List(args) => list(&args),
    }
}
