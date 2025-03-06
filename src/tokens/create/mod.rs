mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error,
};

use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::tokens::create::graphql::generate_token::{generate_token, GenerateToken};
use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select, Sort};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::style;

use termimad::{
    crossterm::style::{Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct TokenCreateArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,

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

    let project_info = project_info_by_slug(&args.slug, &access_token);

    let project_tokens_error_message = format!(
        "Failed to create token for the project with slug '{}'.",
        &args.slug
    );

    let token_name = match Input::<String>::with_theme(&theme())
        .with_prompt("Type a name for the token:")
        .validate_with(|name: &String| -> Result<(), &str> {
            if name.trim().chars().count() == 0 {
                return Err("The token name must contain at least 1 character.");
            } else {
                Ok(())
            }
        })
        .interact()
    {
        Ok(name) => name.trim().to_owned(),

        Err(_) => {
            print_formatted_error(&project_tokens_error_message);
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

    let create_token_response =
        execute_graphql_request::<generate_token::Variables, generate_token::ResponseData>(
            authorization_headers(&access_token),
            GenerateToken::build_query,
            &Client::new(),
            &project_tokens_error_message,
            generate_token::Variables {
                id: project_info.id.to_string(),
                name: token_name.to_owned(),
                expires_at: token_expires_at_value.clone(),
            },
        )
        .create_project_token;

    let success_message_template = minimad::TextTemplate::from(
        r#"
    ##### ✔ Token successfully created
    ID: **${id}**
    Name: **${name}**
    Expires at: **${expires}**

    > *Please make sure to store this token in a safe place.*
    > *If you ever lose or forget this token, you can regenerate it.*
    > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
    > *Press enter to erase the value.*
    "#,
    );

    let mut expander = success_message_template.expander();

    let styled_short_id = format!("{}", &create_token_response.id[..4].with(Color::Green));

    expander
        .set("id", &styled_short_id)
        .set("name", &token_name)
        .set("expires", &options[expires_at_selections]);

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
            style(format!("{}", &create_token_response.value)).with(Color::Green),
            "Copy this token and press 'Enter' to remove it from the screen."
        ))
        .clear(true)
        .report(false)
        .interact()
    {
        Ok(confirmation) => confirmation,

        Err(_) => {
            print_formatted_error("Failed to display the token on the screen. You can see it in the web application or try creating it again.");
            std::process::exit(1);
        }
    };
}
