pub mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    execute_graphql_request::execute_graphql_request,
    format_relative_time::format_relative_time,
    keyring::keyring,
    lengify::lengify,
    pad_to_column_width::pad_to_column_width,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
    table::table,
    take_user_id_from_token::take_user_id_from_token,
};
use crate::projects::list::graphql::team_projects::{team_projects, TeamProjects};
use crate::projects::list::graphql::user_personal_project_team_id::{
    user_personal_project_team_id, UserPersonalProjectTeamId,
};
use clap::Args;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct ProjectsListArgs {
    #[clap(
        short,
        long,
        help = "Team ID (First 4 characters or more are allowed), not required"
    )]
    id: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn list(args: &ProjectsListArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let authorization_headers = authorization_headers(&access_token);

    let user_id = match Uuid::parse_str(&take_user_id_from_token(&access_token)) {
        Ok(uuid) => uuid,

        Err(err) => {
            print_formatted_error(&format!("Invalid user id: {}", err));
            std::process::exit(1);
        }
    };

    let team_id;

    match &args.id {
        Some(id) => {
            team_id = query_full_id(QueryType::Teams, id.clone(), &access_token);
        }

        None => {
            let user_personal_project_team_id_error_message = "Personal projects not found";

            let personal_team = execute_graphql_request::<
                user_personal_project_team_id::Variables,
                user_personal_project_team_id::ResponseData,
            >(
                authorization_headers.clone(),
                UserPersonalProjectTeamId::build_query,
                &client,
                &user_personal_project_team_id_error_message,
                user_personal_project_team_id::Variables {
                    user_id: user_id.clone(),
                },
            )
            .team;

            if let Some(team_data) = personal_team.first() {
                team_id = team_data.id.clone();
            } else {
                print_formatted_error(&user_personal_project_team_id_error_message);
                std::process::exit(1);
            }
        }
    }

    let projects: Vec<team_projects::TeamProjectsProject> =
        execute_graphql_request::<team_projects::Variables, team_projects::ResponseData>(
            authorization_headers.clone(),
            TeamProjects::build_query,
            &client,
            &format!("Projects for team '{}' not found.", &team_id),
            team_projects::Variables {
                id: team_id,
                user_id,
            },
        )
        .project;

    struct ColumnWidthSize {
        id: usize,
        last_usage: usize,
    }

    let mut column_width_size = ColumnWidthSize {
        id: 5,
        last_usage: 10,
    };

    // save the length of the longest column element
    for project in &projects {
        if let Some(usage_history) = project.usage_histories.first() {
            column_width_size.last_usage = column_width_size.last_usage.max(
                format_relative_time(&usage_history.updated_at.to_string())
                    .unwrap()
                    .content()
                    .len(),
            );
        }
    }

    let mut list = Vec::new();
    let indentation = 2;

    for project in projects {
        let last_usage_time = if let Some(usage_history) = project.usage_histories.first() {
            match format_relative_time(&usage_history.updated_at.to_string()) {
                Ok(relative_time) => relative_time,

                Err(_) => {
                    print_formatted_error(
                        "Failed to format the last usage time. Please try again.",
                    );
                    std::process::exit(1);
                }
            }
        } else {
            "never used".to_string().grey()
        };

        list.push(format!(
            "{}{}{}",
            pad_to_column_width(
                format!("#{}", &project.id.to_string()[..4]),
                column_width_size.id + indentation
            )
            .green(),
            lengify(&project.name),
            pad_to_column_width(
                format!("{}", last_usage_time),
                column_width_size.last_usage + indentation
            )
        ))
    }

    table(
        list,
        format!(
            "Showing projects in {} team",
            &team_id.to_string().bold().dark_cyan()
        ),
        "You don't have any projects on this team.",
        format!(
            "{}{}{}",
            pad_to_column_width("ID".to_string(), column_width_size.id + indentation),
            lengify("NAME"),
            pad_to_column_width(
                "LAST USAGE".to_string(),
                column_width_size.last_usage + indentation
            )
        ),
    )
}
