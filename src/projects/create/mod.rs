mod graphql;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, fetch_user_id::fetch_user_id,
    keyring::keyring, print_formatted_error::print_formatted_error,
};
use crate::projects::create::graphql::check_team_name::{check_team_name, CheckTeamName};
use crate::projects::create::graphql::create_project::{create_project, CreateProject};
use clap::Args;
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Sort};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsCreateArgs {
    #[clap(
        short,
        long,
        help = "Project name, if not specified, the user will be prompted to enter the name"
    )]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Team name, if not specified, the project will be added to personal projects"
    )]
    team_name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &ProjectsCreateArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    // Check for the specified team (if not specified, set "Personal projects" by default)
    let team_name = match &args.team_name {
        Some(team) if !team.is_empty() => team.clone(),
        _ => "Personal projects".to_string(),
    };

    let check_team_error_message = format!(
        "Creation failed. The team '{}' not found.",
        team_name.clone()
    );

    // Check the team name in the database
    let team_response =
        execute_graphql_request::<check_team_name::Variables, check_team_name::ResponseData>(
            headers.clone(),
            CheckTeamName::build_query,
            &client,
            &check_team_error_message,
            check_team_name::Variables {
                user_id: fetch_user_id(&access_token),
                name: team_name.clone(),
            },
        )
        .team;

    let team_id = match team_response.len() {
        1 => team_response[0].id,

        _ => {
            print_formatted_error(&check_team_error_message);
            std::process::exit(1);
        }
    };

    let project_name = match &args.name {
        Some(name) => {
            if name.trim().chars().count() < 2 {
                print_formatted_error(
                    "Creation failed. The project name must be at least 2 characters long.",
                );

                std::process::exit(1);
            }

            name.clone()
        }

        None => {
            match Input::with_theme(&theme())
                .with_prompt("Type a project name:")
                .validate_with(|name: &String| -> Result<(), &str> {
                    if name.trim().chars().count() < 2 {
                        return Err("The project name must be at least 2 characters long.");
                    } else {
                        Ok(())
                    }
                })
                .interact()
            {
                Ok(new_name) => new_name,

                Err(_) => {
                    print_formatted_error("Creation failed. Failed to read the project name.");
                    std::process::exit(1);
                }
            }
        }
    };

    let create_project_error_message = format!(
        "Creation failed. Failed to create a project with the name '{}'.",
        project_name.clone()
    );

    // Create a project
    let project_response =
        execute_graphql_request::<create_project::Variables, create_project::ResponseData>(
            headers.clone(),
            CreateProject::build_query,
            &client,
            &create_project_error_message,
            create_project::Variables {
                object: create_project::CreateProjectInput {
                    name: project_name.clone(),
                    active_team_id: team_id,
                    icon: "star".to_string(), // Default icon
                },
            },
        )
        .create_project;

    let project = match project_response {
        Some(project) => project,

        None => {
            print_formatted_error(&create_project_error_message);
            std::process::exit(1);
        }
    };

    // MD template for the project creation message
    let text_template = minimad::TextTemplate::from(
        r#"
        ##### ✔ Project successfully created
        ${description}
        **Project link**: ${project-link}
        **Project ID for CLI**: ${short-project-id}

        > *Please make sure to store this token in a safe place.*
        > *If you ever lose or forget this token, you can regenerate it.*
        > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
        "#,
    );

    let mut expander = text_template.expander();
    let project_id = project.id.to_string();

    let description = if &team_name == "Personal projects" {
        format!(
            "You created the project named '{}' as a personal.",
            &project_name
        )
    } else {
        format!(
            "You created the project named '{}' within the '{}' team.",
            &project_name, &team_name
        )
    };

    let project_link = style(format!(
        "{}/projects/{}",
        Config::new().webapp_url,
        &project_id.to_string().replace("-", "")
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    let styled_short_id = format!("{}", &project_id[..4].with(Color::Green));
    let styled_project_id = format!("{}", &project_id.with(Color::Green));

    expander
        .set("description", &description)
        .set("project-link", &project_link)
        .set("project-id", &styled_project_id)
        .set("short-project-id", &styled_short_id);

    let mut skin = MadSkin::default();
    skin.headers[4].set_fg(Color::Green);
    skin.print_expander(expander);

    let sort_theme = ColorfulTheme {
        active_item_style: Style::new(),
        picked_item_prefix: dialoguer::console::style("".to_string()),
        unpicked_item_prefix: dialoguer::console::style("".to_string()),
        prompt_prefix: dialoguer::console::style("".to_string()),
        ..ColorfulTheme::default()
    };

    match Sort::with_theme(&sort_theme)
        .item(format!(
            "{} '{}'. {}",
            "Your token:",
            style(format!("{}", &project.token)).with(Color::Green),
            "Copy this token and press 'Enter' to remove it from the screen."
        ))
        .clear(true)
        .report(false)
        .interact()
    {
        Ok(confirmation) => confirmation,

        Err(_) => {
            print_formatted_error(
                &format!("Creation failed. Failed to display information about the created project with the name '{}'.", &project_name),
            );

            std::process::exit(1);
        }
    };
}
