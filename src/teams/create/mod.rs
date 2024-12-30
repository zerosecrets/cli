mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error, take_user_id_from_token::take_user_id_from_token,
};
use crate::teams::create::graphql::create_team::{create_team, CreateTeam};
use crate::teams::create::graphql::user_info_and_team_names::{
    user_info_and_team_names, UserInfoAndTeamNames,
};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use regex::Regex;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct TeamsCreateArgs {
    #[clap(short, long, help = "The name of the team, not required")]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

fn validate_team_name(name: &String, existing_names: &Vec<String>) -> Result<(), String> {
    let min_length = 2;
    let name_regex = Regex::new(r"^[\w -]+$").unwrap();

    if name.trim().len() < min_length {
        return Err(format!(
            "Team name must be at least {} characters",
            min_length
        ));
    }

    if !name_regex.is_match(name.trim()) {
        return Err("Only a-z, 0-9, ' ', '_', and '-' are allowed".to_string());
    }

    if existing_names.contains(&name.trim().to_string()) {
        return Err("Team name is already in use".to_string());
    }

    Ok(())
}

pub fn create(args: &TeamsCreateArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let user_id = match Uuid::parse_str(&take_user_id_from_token(&access_token)) {
        Ok(uuid) => uuid,

        Err(err) => {
            print_formatted_error(&format!("Invalid user id: {}", err));
            std::process::exit(1);
        }
    };

    let user_info = execute_graphql_request::<
        user_info_and_team_names::Variables,
        user_info_and_team_names::ResponseData,
    >(
        headers.clone(),
        UserInfoAndTeamNames::build_query,
        &client,
        "Failed to retrieve team names",
        user_info_and_team_names::Variables { user_id },
    );

    let subscription_error = "Creation failed. Failed to retrieve subscription info.";

    match user_info.user_by_pk {
        Some(user) => match user.user_subscription {
            Some(subscription) => match subscription.subscription_plan {
                user_info_and_team_names::subscriptionPlanEnum_enum::free => {
                    print_formatted_error(
                        "Creation failed. A paid subscription is required to create a team.",
                    );

                    std::process::exit(1);
                }

                _ => (),
            },

            None => {
                print_formatted_error(subscription_error);
                std::process::exit(1);
            }
        },

        None => {
            print_formatted_error(subscription_error);
            std::process::exit(1);
        }
    };

    let existing_team_names = user_info
        .team
        .iter()
        .map(|team| team.name.to_owned())
        .collect::<Vec<String>>();

    let team_name = if let Some(name) = &args.name {
        if let Err(err) = validate_team_name(&name, &existing_team_names) {
            eprintln!("Validation error: {}", err);
            print_formatted_error(&format!("Validation error: {}", err));
            std::process::exit(1);
        } else {
            name.trim().to_owned()
        }
    } else {
        match Input::with_theme(&theme())
            .with_prompt("Type a name for the team:")
            .validate_with(|input: &String| -> Result<(), String> {
                validate_team_name(input, &existing_team_names).map_err(|e| e.as_str().to_owned())
            })
            .interact()
        {
            Ok(name) => name.trim().to_owned(),

            Err(_) => {
                print_formatted_error("Failed to read team name input.");
                std::process::exit(1);
            }
        }
    };

    let team_id = execute_graphql_request::<create_team::Variables, create_team::ResponseData>(
        headers.clone(),
        CreateTeam::build_query,
        &client,
        &format!(
            "Creation failed. Failed to create a team with the name '{}'.",
            team_name
        ),
        create_team::Variables { team_name },
    )
    .create_team
    .id;

    let text_template = minimad::TextTemplate::from(
        r#"
        ##### **✔ Team successfully created**
        Team link: ${team-link}
        Team ID: **${team-id}**
        "#,
    );

    let mut expander = text_template.expander();

    let team_link = style(format!(
        "{}/teams/{}",
        Config::new().webapp_url,
        &team_id.replace("-", "")
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    expander
        .set("team-link", &team_link)
        .set("team-id", &team_id);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Green);
    skin.print_expander(expander);
}
