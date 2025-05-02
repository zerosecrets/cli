mod graphql;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
};
use crate::teams::common::team_info::team_info;
use clap::Args;
use dialoguer::Input;
use graphql::delete_team::{delete_team, DeleteTeam};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct TeamsDeleteArgs {
    #[clap(short, long, help = "Team slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn delete(args: &TeamsDeleteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let authorization_headers = authorization_headers(&access_token);
    let client = Client::new();
    let team_info = team_info(&access_token, args.slug.clone());

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm deletion:",
            args.slug.clone().green()
        ))
        .interact_text()
        .expect("Deletion failed. Failed to read the user's reply.");

    if input != args.slug {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let remove_team_error_message = format!(
        "Deletion failed. Failed to delete the team with slug '{}'.",
        args.slug
    );

    execute_graphql_request::<delete_team::Variables, delete_team::ResponseData>(
        authorization_headers.clone(),
        DeleteTeam::build_query,
        &client,
        &remove_team_error_message,
        delete_team::Variables {
            id: team_info.id.to_string(),
        },
    );

    println!("{} {}", "✔".green(), "Team successfully deleted");
}
