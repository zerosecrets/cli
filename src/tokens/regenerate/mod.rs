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
use crate::tokens::regenerate::graphql::regenerate_token::{regenerate_token, RegenerateToken};
use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Select, Sort};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::style;
use termimad::{
    crossterm::style::{Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct TokenRegenerateArgs {
    #[clap(
        short,
        long,
        help = "Token ID (First 4 characters or more are allowed)"
    )]
    id: String,

    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn regenerate(args: &TokenRegenerateArgs) {
    let items_per_page = Config::new().items_per_page;

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let token_id = query_full_id(QueryType::Tokens, args.id.clone(), &access_token);

    let regenerate_token_error_message = format!(
        "Failed to regenerate a token for the project with ID '{}'.",
        &args.id
    );

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

    let regenerate_token_response =
        execute_graphql_request::<regenerate_token::Variables, regenerate_token::ResponseData>(
            authorization_headers(&access_token),
            RegenerateToken::build_query,
            &Client::new(),
            &regenerate_token_error_message,
            regenerate_token::Variables {
                id: token_id.to_string(),
                expires_at: token_expires_at_value.clone(),
            },
        )
        .regenerate_project_token;

    let new_token_value = match regenerate_token_response.token_value {
        Some(token) => token,
        None => {
            print_formatted_error("Failed to regenerate a token.");
            std::process::exit(1);
        }
    };

    let success_message_template = minimad::TextTemplate::from(
        r#"
    ##### ✔ Token successfully regenerated!
    ID: **${id}**
    Expires at: **${expires}**

    > *Please make sure to store this token in a safe place.*
    > *If you ever lose or forget this token, you can regenerate it.*
    > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
    > *Press enter to erase the value.*
    "#,
    );

    let mut expander = success_message_template.expander();

    let styled_short_id = format!("{}", &token_id.to_string()[..4].with(Color::Green));

    expander
        .set("id", &styled_short_id)
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
            style(format!("{}", &new_token_value)).with(Color::Green),
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
