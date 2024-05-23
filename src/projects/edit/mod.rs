mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    fetch_user_id::fetch_user_id,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::projects::edit::graphql::project_info::{project_info, ProjectInfo};
use crate::projects::edit::graphql::update_project_description::{
    update_project_description, UpdateProjectDescription,
};
use crate::projects::edit::graphql::update_project_name::{update_project_name, UpdateProjectName};
use clap::Args;
use dialoguer::{Confirm, Input};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::{style, Color, Stylize};

#[derive(Args, Debug)]
pub struct ProjectsEditArgs {
    #[clap(
        short,
        long,
        help = "Project ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(short, long, help = "Project name, not required")]
    name: Option<String>,
    #[clap(short, long, help = "Project description, not required")]
    description: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn edit(args: &ProjectsEditArgs) -> () {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let user_id = fetch_user_id(&access_token);
    let mut name = args.name.clone();
    let mut description = args.description.clone();

    // If name is passed as an argument, it must be at least 2 characters long
    if name.clone().is_some() && name.clone().unwrap().chars().count() < 2 {
        print_formatted_error(
            "Editing failed. The project name must be at least 2 characters long.",
        );
        std::process::exit(1);
    }

    let project_id = query_full_id(QueryType::Project, args.id.clone(), &access_token);
    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let project_info =
        match execute_graphql_request::<project_info::Variables, project_info::ResponseData>(
            headers.clone(),
            ProjectInfo::build_query,
            &client,
            "Failed to retrieve project info.",
            project_info::Variables {
                project_id: project_id.clone(),
            },
        )
        .project_by_pk
        {
            Some(project_info) => project_info,

            None => {
                print_formatted_error(&format!(
                    "Editing failed. Project '{}' was not found.",
                    args.id.clone()
                ));
                std::process::exit(1);
            }
        };

    if project_info.owner_user_id != user_id {
        print_formatted_error("Editing failed. You are not the owner of this project.");
        std::process::exit(1);
    }

    // If no arguments are passed, the user will be prompted to enter a new name and description of the project
    if name.is_none() && description.is_none() {
        name = match Input::with_theme(&theme())
            .with_prompt("Type a new project name:")
            .validate_with(|name: &String| -> Result<(), &str> {
                if name.trim().chars().count() < 2 {
                    return Err("The project name must be at least 2 characters long.");
                } else {
                    Ok(())
                }
            })
            .default(project_info.name.clone())
            .interact()
        {
            Ok(new_name) => Some(new_name),

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the project name.");
                std::process::exit(1);
            }
        };

        let is_confirmed = match Confirm::new()
            .with_prompt("Do you want to update the project description?")
            .interact()
        {
            Ok(confirmation) => confirmation,

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the user confirmation.");
                std::process::exit(1);
            }
        };

        if is_confirmed {
            description = match Input::with_theme(&theme())
                .with_prompt("Type a new project description:")
                .interact()
            {
                Ok(new_description) => Some(new_description),

                Err(_) => {
                    print_formatted_error("Editing failed. Failed to get the project description.");
                    std::process::exit(1);
                }
            };
        }
    }

    if let Some(name) = name {
        let update_project_name_error_message = format!(
            "Editing failed. Failed to update the name of the project '{}'.",
            args.id.clone()
        );

        match execute_graphql_request::<
            update_project_name::Variables,
            update_project_name::ResponseData,
        >(
            headers.clone(),
            UpdateProjectName::build_query,
            &client,
            &update_project_name_error_message,
            update_project_name::Variables {
                user_id: user_id.clone(),
                project_id: project_id.clone(),
                project_name: name.clone(),
            },
        )
        .update_project
        {
            Some(data) => {
                if data.affected_rows != 1 {
                    print_formatted_error(&update_project_name_error_message);
                    std::process::exit(1);
                }
            }

            None => {
                print_formatted_error(&update_project_name_error_message);
                std::process::exit(1);
            }
        };
    }

    if let Some(description) = description {
        let update_project_description_error_message = format!(
            "Editing failed. Failed to update the description of the project '{}'.",
            args.id.clone()
        );

        match execute_graphql_request::<
            update_project_description::Variables,
            update_project_description::ResponseData,
        >(
            headers,
            UpdateProjectDescription::build_query,
            &client,
            &update_project_description_error_message,
            update_project_description::Variables {
                user_id: user_id.clone(),
                project_id: project_id.clone(),
                project_description: description.clone(),
            },
        )
        .update_project
        {
            Some(data) => {
                if data.affected_rows != 1 {
                    print_formatted_error(&update_project_description_error_message);
                    std::process::exit(1);
                }
            }

            None => {
                print_formatted_error(&update_project_description_error_message);
                std::process::exit(1);
            }
        };
    }

    println!(
        "{} {}",
        "✔".green(),
        "The project has been successfully updated."
    );

    println!(
        "{}",
        style(format!(
            "{}/projects/{}",
            Config::new().webapp_url,
            project_id.to_string().replace("-", "")
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
    );
}
