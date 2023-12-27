use clap::{Parser, Subcommand};
/// ## Login
///
///  The `login` function initiates the login process for a user in the Zero system. Asks the user
///  to open the provided URL in their web browser and authorize. Validates the received code,
///  receives and saves the access token.
///
mod login;
/// ## Logout
///
///  The `logout` function is used to log a user out of their Zero account. Deletes the stored
///  access token from the device.
///
mod logout;
use self::login::AuthLoginArgs;

#[derive(Parser, Debug)]
pub struct AuthCli {
    #[clap(subcommand)]
    commands: AuthCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Authentication and authorization")]
enum AuthCommands {
    #[clap(about = "Login to the platform")]
    Login(AuthLoginArgs),
    #[clap(about = "Logout from the platform")]
    Logout,
}

pub fn match_command(input: &AuthCli) {
    match &input.commands {
        AuthCommands::Login(args) => login::login(args),
        AuthCommands::Logout => logout::logout(),
    }
}
