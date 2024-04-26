/// ## Tokens Delete
///
/// The `delete` module contains subcommands for deleting project tokens.
///
mod delete;
/// ## Tokens List
///
///  The `Tokens list` function displays information about the project tokens
///
mod list;
/// ## Create Tokens
///
///  The `Create token` function creates a new token for the project
///
mod create;
use crate::tokens::delete::{delete, TokensDeleteArgs};
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
    #[clap(about = "Delete a project token")]
    Delete(TokensDeleteArgs),
    #[clap(about = "List all project token")]
    List(TokenListArgs),
    #[clap(about = "Create project token")]
    Create(TokenCreateArgs),
}

pub fn match_command(input: &TokenCli) {
    match &input.commands {
        TokenCommands::Delete(args) => delete(&args),
        TokenCommands::List(args) => list(&args),
        TokenCommands::Create(args) => create(&args),
    }
}
