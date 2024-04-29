/// ## Create Tokens
///
///  The `Create token` function creates a new token for the project
///
mod create;
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
/// ## Regenerate Tokens
///
///  The `Regenerate token` function regenerates the token for the project
///
mod regenerate;
use crate::tokens::create::{create, TokenCreateArgs};
use crate::tokens::delete::{delete, TokensDeleteArgs};
use crate::tokens::list::{list, TokenListArgs};
use crate::tokens::regenerate::{regenerate, TokenRegenerateArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct TokenCli {
    #[clap(subcommand)]
    commands: TokenCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "View a project token list")]
pub enum TokenCommands {
    #[clap(about = "Create a project token")]
    Create(TokenCreateArgs),
    #[clap(about = "Delete a project token")]
    Delete(TokensDeleteArgs),
    #[clap(about = "List all project token")]
    List(TokenListArgs),
    #[clap(about = "Regenerate a project token")]
    Regenerate(TokenRegenerateArgs),
}

pub fn match_command(input: &TokenCli) {
    match &input.commands {
        TokenCommands::Create(args) => create(&args),
        TokenCommands::Delete(args) => delete(&args),
        TokenCommands::List(args) => list(&args),
        TokenCommands::Regenerate(args) => regenerate(&args),
    }
}
