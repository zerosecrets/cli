pub mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    graphql::generate_secret_sharing_url::{generate_secret_sharing_url, GenerateSecretSharingUrl},
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::secrets::share::graphql::user_secret::{user_secret, UserSecret};
use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::{MultiSelect, Password, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::{style, Color, Stylize};

#[derive(Args, Debug)]
pub struct SecretShareArgs {
    #[clap(
        short,
        long,
        help = "Secret ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn share(args: &SecretShareArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let secret_id = query_full_id(QueryType::UserSecret, args.id.clone(), &access_token);
    let user_secrets_error_message = format!(
        "Sharing error. Failed to retrieve secret with ID '{}'.",
        &args.id
    );

    let secret_details =
        match execute_graphql_request::<user_secret::Variables, user_secret::ResponseData>(
            authorization_headers(&access_token),
            UserSecret::build_query,
            &Client::new(),
            &user_secrets_error_message,
            user_secret::Variables { id: secret_id },
        )
        .user_secret_by_pk
        {
            Some(secret) => secret,

            None => {
                print_formatted_error(&user_secrets_error_message);
                std::process::exit(1);
            }
        };

    let passphrase = match Password::with_theme(&theme())
        .with_prompt("Type a passphrase of at least 6 character:")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().chars().count() >= 6 {
                Ok(())
            } else {
                print_formatted_error("Passphrase must be at least 6 character long.");
                std::process::exit(1);
            }
        })
        .interact()
    {
        Ok(passphrase) => passphrase,

        Err(_) => {
            print_formatted_error("Sharing error. Failed to read the passphrase.");
            std::process::exit(1);
        }
    };

    let options = ["5 min", "30 min", "1 hour", "1 day"];
    let items_per_page = Config::new().items_per_page;

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
            print_formatted_error("Sharing error. Failed to read the expiration time.");
            std::process::exit(1);
        }
    };

    let expires_at_minutes = match options[expires_at_selections] {
        "5 min" => 5,
        "30 min" => 30,
        "1 hour" => 60,
        "1 day" => 1440,

        _ => {
            print_formatted_error("Sharing error. Failed to read the expiration time.");
            std::process::exit(1);
        }
    };

    let secret_fields_selection = match MultiSelect::with_theme(&theme())
        .with_prompt(format!(
            "Pick secret fields: {}",
            "Use <Up>/<Down> to navigate, <Space> to select, and <Enter> to confirm".dark_grey()
        ))
        .items(
            &secret_details
                .fields
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
        )
        .max_length(items_per_page)
        .interact()
    {
        Ok(secret_fields_selection) => {
            if secret_fields_selection.is_empty() {
                print_formatted_error("Sharing error. You must select at least one secret.");
                std::process::exit(1);
            } else {
                secret_fields_selection
            }
        }

        Err(_) => {
            print_formatted_error("Sharing error. Failed to read the secrets.");
            std::process::exit(1);
        }
    };

    let selected_secrets: Vec<String> = secret_fields_selection
        .iter()
        .filter_map(|&index| secret_details.fields.get(index))
        .map(|field| field.id.to_string())
        .collect();

    let secrets_sharing_url = execute_graphql_request::<
        generate_secret_sharing_url::Variables,
        generate_secret_sharing_url::ResponseData,
    >(
        authorization_headers(&access_token),
        GenerateSecretSharingUrl::build_query,
        &Client::new(),
        &format!(
            "Sharing error. Failed to generate secret sharing url for project '{}'.",
            &secret_details.project_id
        ),
        generate_secret_sharing_url::Variables {
            expires_at: (Utc::now() + Duration::minutes(expires_at_minutes)).to_rfc3339(),
            pass_phrase: passphrase,
            secrets_field_ids: selected_secrets,
        },
    )
    .generate_secret_sharing_url
    .url;

    println!(
        "Your link for the shared secret '{}' with the name '{}':",
        &secret_details.id.to_string()[..4].dark_cyan(),
        &secret_details.name,
    );

    println!(
        "{}",
        style(secrets_sharing_url).with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
    );
}
