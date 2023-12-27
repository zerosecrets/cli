mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use clap::Args;
use graphql::team::{team, Team};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    MadSkin,
};

#[derive(Args, Debug)]
pub struct TeamViewArgs {
    #[clap(short, long, help = "Team ID (First 4 characters or more are allowed)")]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn view(args: &TeamViewArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let team_id = query_full_id(QueryType::Teams, args.id.clone(), &access_token);

    let team_error_message = format!(
        "Failed to retrieve team details with ID '{}'.",
        &team_id.to_string()[..4]
    );

    let team_details = match execute_graphql_request::<team::Variables, team::ResponseData>(
        authorization_headers(&access_token),
        Team::build_query,
        &Client::new(),
        &team_error_message,
        team::Variables { id: team_id },
    )
    .team_by_pk
    {
        Some(data) => data,

        None => {
            print_formatted_error(&team_error_message);
            std::process::exit(1);
        }
    };

    let members = team_details
        .members
        .iter()
        .map(|member| format!("  {}\n", member.member.name))
        .collect::<Vec<String>>();

    let skin = MadSkin {
        ..Default::default()
    };

    let markdown_text = format!(
        "**URL**: {} \n**Name**: {} \n**Description**: {}\n**Owner**: {}\n**Members**:\n{}",
        style(format!(
            "{}/teams/{}",
            Config::new().webapp_url,
            team_details.id
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        }),
        &team_details.name,
        match &team_details.description {
            Some(description) => description,
            None => "-",
        },
        match &team_details.owner {
            Some(owner) => &owner.name,
            None => "Owner not found",
        },
        &members.join(""),
    );

    skin.print_text(&markdown_text);
}
