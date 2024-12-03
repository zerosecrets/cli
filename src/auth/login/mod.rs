pub mod graphql;
use crate::auth::login::graphql::cli_access_tokens::{cli_access_tokens, CliAccessTokens};
use crate::auth::login::graphql::user_by_pk::{user_by_pk, UserByPk};
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error, take_user_id_from_token::take_user_id_from_token,
};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::{header, Client};
use std::io;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct AuthLoginArgs {
    #[clap(
        short,
        long,
        help = "If true - returns the access token to the console, if false or not specified - saves the access token to the keychain"
    )]
    console_output: Option<bool>,
}

pub fn login(args: &AuthLoginArgs) {
    let config = Config::new();
    let webapp_cli_auth_url = config.webapp_cli_auth_url;
    let client = Client::new();

    let is_console_output = match &args.console_output {
        Some(token) => token.clone(),
        _ => false,
    };

    println!(
        "Press `Enter` to open tryzero.com in your browser or type `q` to quit.\n{}",
        &webapp_cli_auth_url
    );

    // If the user pressed Enter, open the authorization page in the browser
    // and ask him to enter the code from the browser
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Authorization error. Please try again.");

        input = input.trim().to_string();

        match input.as_str() {
            "" => {
                match open::that(&webapp_cli_auth_url) {
                    Ok(_) => (),
                    Err(_) => {
                        print_formatted_error("Authorization error. Failed to open the browser.")
                    }
                };

                let code = match Input::with_theme(&theme())
                    .with_prompt("Type the code shown in the browser:")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.trim().chars().count() == 32 {
                            Ok(())
                        } else {
                            Err("The code must be 32 characters long.")
                        }
                    })
                    .interact()
                {
                    Ok(value) => value.trim().to_string(),

                    Err(_) => {
                        print_formatted_error("Authorization error. Please try again.");
                        std::process::exit(1);
                    }
                };

                // Check the code and get the user ID and its access token
                let user_auth = execute_graphql_request::<cli_access_tokens::Variables, cli_access_tokens::ResponseData>(
                    header::HeaderMap::new(),
                    CliAccessTokens::build_query,
                    &client,
                    "Failed to retrieve access token, please check if the code is correct and try again.",
                    cli_access_tokens::Variables::new(code),
                ).cli_access_tokens;

                let user_id =
                    match Uuid::parse_str(&take_user_id_from_token(&user_auth.access_token)) {
                        Ok(uuid) => uuid,

                        Err(err) => {
                            print_formatted_error(&format!("Invalid user id: {}", err));
                            std::process::exit(1);
                        }
                    };

                let user_info_error_message =
                    "Authorization error. Failed to retrieve a user info.";

                // Send a request for user info
                let user_info_response =
                    execute_graphql_request::<user_by_pk::Variables, user_by_pk::ResponseData>(
                        authorization_headers(&user_auth.access_token),
                        UserByPk::build_query,
                        &client,
                        user_info_error_message,
                        user_by_pk::Variables { id: user_id },
                    )
                    .user_by_pk;

                let user_info = match user_info_response {
                    Some(user_info) => user_info,

                    None => {
                        print_formatted_error(user_info_error_message);
                        std::process::exit(1);
                    }
                };

                match is_console_output {
                    true => {
                        println!("{} You are authorized as {}", "✓".green(), user_info.name);

                        println!(
                            "{}",
                            "Be careful! Do not share your access token with anyone.".yellow()
                        );

                        println!(
                            "{} {}",
                            "Your access token is:".bold(),
                            user_auth.access_token
                        );
                        std::process::exit(0);
                    }

                    false => {
                        keyring::set("access_token", &user_auth.access_token);
                        println!("{} You are authorized as {}", "✓".green(), user_info.name);
                        std::process::exit(0);
                    }
                }
            }

            "q" | "Q" => {
                std::process::exit(0);
            }

            _ => {
                println!("You've entered: {}. Press only ENTER to continue.", input);
            }
        }
    }
}
