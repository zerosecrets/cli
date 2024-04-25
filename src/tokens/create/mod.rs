mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::tokens::create::graphql::generate_token::{generate_token, GenerateToken};
use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::{Input, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use termimad::{crossterm::style::Color, minimad, MadSkin};

#[derive(Args, Debug)]
pub struct TokenCreateArgs {
    #[clap(
        short,
        long,
        help = "Project ID (First 4 characters or more are allowed)"
    )]
    id: String,

    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &TokenCreateArgs) {
    let items_per_page = Config::new().items_per_page;

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_id = query_full_id(QueryType::Project, args.id.clone(), &access_token);

    let project_tokens_error_message = format!(
        "Failed to create token for the project with ID '{}'.",
        &args.id
    );

    let name_error_message = "Creation failed.";

    let token_name = match Input::<String>::with_theme(&theme())
        .with_prompt("Type a name for the token:")
        .interact()
    {
        Ok(name) if !name.trim().is_empty() => name.trim().to_owned(),

        Ok(_) => {
            print_formatted_error(name_error_message);
            std::process::exit(1);
        }

        Err(_) => {
            print_formatted_error(name_error_message);
            std::process::exit(1);
        }
    };

    let options = ["Endless", "7 days", "30 days", "60 days", "90 days"];
    let read_expires_time_error = "Creating failed. Failed to read expiration time.";

    let expires_at_selections = match Select::with_theme(&theme())
        .with_prompt(format!(
            "Expires in: {}",
            "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
        ))
        .default(0)
        .items(
            &options
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>(),
        )
        .max_length(items_per_page)
        .interact()
    {
        Ok(expires_at_selections) => expires_at_selections,

        Err(_) => {
            print_formatted_error(read_expires_time_error);
            std::process::exit(1);
        }
    };

    let token_expires_at_value = match options[expires_at_selections] {
        "Endless" => None,
        "7 days" => Some((Utc::now() + Duration::days(7)).to_string()),
        "30 days" => Some((Utc::now() + Duration::days(30)).to_string()),
        "60 days" => Some((Utc::now() + Duration::days(60)).to_string()),
        "90 days" => Some((Utc::now() + Duration::days(90)).to_string()),

        _ => {
            print_formatted_error("Creation failed. Failed to read expiration time.");
            std::process::exit(1);
        }
    };

    let token_expires_at = match &token_expires_at_value {
        Some(value) => value,

        None => {
            print_formatted_error(read_expires_time_error);
            std::process::exit(1);
        }
    };

    let create_token_response =
        execute_graphql_request::<generate_token::Variables, generate_token::ResponseData>(
            authorization_headers(&access_token),
            GenerateToken::build_query,
            &Client::new(),
            &project_tokens_error_message,
            generate_token::Variables {
                id: project_id.to_string(),
                name: token_name.to_owned(),
                expires_at: token_expires_at_value.clone(),
            },
        )
        .create_project_token;

    let token_value = match create_token_response.token_value {
        Some(token) => token,

        None => {
            print_formatted_error(&project_tokens_error_message);
            std::process::exit(1);
        }
    };

    let success_message_template = minimad::TextTemplate::from(
        r#"
    ##### ✔ Token successfully created
    Name: **${name}**
    Expires at: **${expires}**
    Token: **${token}**

    > *Please make sure to store this token in a safe place.*
    > *If you ever lose or forget this token, you can regenerate it.*
    > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
    > *Press enter to erase the value.*
    "#,
    );

    let mut expander = success_message_template.expander();s

    expander
        .set("name", &token_name)
        .set("expires", &options[expires_at_selections])
        .set("token", &token_value);

    let mut skin = MadSkin::default();
    skin.headers[4].set_fg(Color::Green);
    skin.print_expander(expander);
}
