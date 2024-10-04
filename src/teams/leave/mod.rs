mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::teams::common::team_info::team_info;
use clap::Args;
use dialoguer::Input;
use graphql::remove_user_from_team::{remove_user_from_team, RemoveUserFromTeam};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct TeamLeaveArgs {
    #[clap(short, long, help = "Team ID (First 4 characters or more are allowed)")]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn leave(args: &TeamLeaveArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let team_id = query_full_id(QueryType::Teams, args.id.clone(), &access_token);

    let user_id = match Uuid::parse_str(&keyring::get("user_id")) {
        Ok(uuid) => uuid,

        Err(err) => {
            print_formatted_error(&format!("Invalid user id: {}", err));
            std::process::exit(1);
        }
    };

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm leaving the team:",
            &team_id.to_string()[..4]
        ))
        .interact_text()
        .expect("Failed to read the user's reply.");

    if input != team_id.to_string()[..4] {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let team_info = team_info(&access_token, team_id);
    let leave_team_error_message = "Failed to leave the team.";

    match execute_graphql_request::<
        remove_user_from_team::Variables,
        remove_user_from_team::ResponseData,
    >(
        authorization_headers(&access_token),
        RemoveUserFromTeam::build_query,
        &Client::new(),
        leave_team_error_message,
        remove_user_from_team::Variables {
            team_id: team_info.id.to_string(),
            user_id: user_id.to_string(),
            owner_team_user_id: team_info.owner_user_id.to_string(),
        },
    )
    .remove_user_from_team
    .success
    {
        true => {
            println!(
                "{} You have successfully left the '{}' team.",
                "✔".green(),
                team_info.name
            );
        }

        _ => {
            print_formatted_error(&leave_team_error_message);
            std::process::exit(1);
        }
    }
}
