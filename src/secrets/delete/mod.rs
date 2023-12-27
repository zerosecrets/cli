mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::secrets::delete::graphql::delete_user_secret::{delete_user_secret, DeleteUserSecret};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct SecretsDeleteArgs {
    #[clap(
        short,
        long,
        help = "Secret ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn delete(args: &SecretsDeleteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let secret_id = query_full_id(QueryType::UserSecret, args.id.clone(), &access_token);

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm deletion:",
            &secret_id.to_string()[..4]
        ))
        .interact_text()
        .expect("Deletion failed. Failed to read user input.");

    if input != secret_id.to_string()[..4] {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let delete_secret_error_message = format!(
        "Deletion failed. Failed to delete the secret with ID '{}'.",
        args.id.clone()
    );

    let delete_secret_response =
        execute_graphql_request::<delete_user_secret::Variables, delete_user_secret::ResponseData>(
            authorization_headers(&access_token),
            DeleteUserSecret::build_query,
            &Client::new(),
            &delete_secret_error_message,
            delete_user_secret::Variables { id: secret_id },
        )
        .delete_user_secret_by_pk;

    if delete_secret_response.is_none() {
        print_formatted_error(&delete_secret_error_message);
        std::process::exit(1);
    }

    println!("{} {}", "✔".green(), "Secret successfully deleted");
}
