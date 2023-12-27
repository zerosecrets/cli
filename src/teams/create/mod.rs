mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, fetch_user_id::fetch_user_id,
    keyring::keyring, print_formatted_error::print_formatted_error,
};
use crate::teams::create::graphql::create_team::{create_team, CreateTeam};
use crate::teams::create::graphql::user_info_and_team_names::{
    user_info_and_team_names, UserInfoAndTeamNames,
};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

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

pub fn create(args: &TeamsCreateArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);
    let user_id = fetch_user_id(&access_token);

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

    let user_team_names = user_info
        .team
        .iter()
        .map(|team| team.name.to_owned())
        .collect::<Vec<String>>();

    let team_name_error_message = "Creation failed.";
    let team_name_unique_error_message = "Creation failed. The team name must be unique.";

    let team_name = if args.name.is_some() {
        match &args.name {
            Some(name) => {
                if user_team_names.contains(&name.trim().to_string()) {
                    print_formatted_error(team_name_unique_error_message);
                    std::process::exit(1);
                } else {
                    name.trim().to_owned()
                }
            }

            None => {
                print_formatted_error(team_name_error_message);
                std::process::exit(1);
            }
        }
    } else {
        match Input::with_theme(&theme())
            .with_prompt("Type a name for the team:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if user_team_names.contains(&input.trim().to_string()) {
                    return Err(team_name_unique_error_message);
                }

                return Ok(());
            })
            .interact()
        {
            Ok(name) => name.trim().to_owned(),

            Err(_) => {
                print_formatted_error(team_name_error_message);
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
    .team_id;

    let text_template = minimad::TextTemplate::from(
        r#"
        ##### **✔ Team successfully created**
        Team link: ${team-link}
        Team ID: **${team-id}**
        "#,
    );

    let mut expander = text_template.expander();

    let team_link = style(format!("{}/teams/{}", Config::new().webapp_url, &team_id))
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
