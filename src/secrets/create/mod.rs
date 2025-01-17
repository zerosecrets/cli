mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config, execute_graphql_request::execute_graphql_request, keyring::keyring, print_formatted_error::print_formatted_error, query_full_id::{query_full_id, QueryType}, slugify::slugify_prompt, validate_secret_field_name::validate_secret_field_name, validate_secret_name::validate_secret_name, vendors::Vendors
};

use crate::secrets::create::graphql::create_secret::{create_secret, CreateSecret};
use crate::secrets::create::graphql::secret_names::{secret_names, SecretNames};
use clap::Args;
use dialoguer::{Confirm, Input, Password, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;

use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct SecretsCreateArgs {
    #[clap(
        short,
        long,
        help = "Project ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Secret name, not required, if not provided will be prompted"
    )]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &SecretsCreateArgs) {
    let config = Config::new();

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);
    let project_id = query_full_id(QueryType::Project, args.id.clone(), &access_token);

    let secret_names =
        match execute_graphql_request::<secret_names::Variables, secret_names::ResponseData>(
            headers.clone(),
            SecretNames::build_query,
            &client,
            "Failed to retrieve secret names",
            secret_names::Variables { project_id },
        )
        .project_by_pk
        {
            Some(token_by_pk) => token_by_pk
                .user_secrets
                .iter()
                .map(|secret| secret.name.clone())
                .collect(),

            None => Vec::new(),
        };

    let secret_name = match &args.name {
        Some(name) => match validate_secret_name(&name, "", &secret_names) {
            Ok(_) => name.clone(),

            Err(error) => {
                print_formatted_error(error);
                std::process::exit(1);
            }
        },

        None => {
            match Input::with_theme(&theme())
                .with_prompt("Type a name for the secret:")
                .validate_with(|input: &String| -> Result<(), &str> {
                    return validate_secret_name(&input, "", &secret_names);
                })
                .interact()
            {
                Ok(name) => name,

                Err(_) => {
                    print_formatted_error("Creation failed. Failed to get a secret name.");
                    std::process::exit(1);
                }
            }
        }
    };

    let vendors = Vendors::new().prettified_vendor_options;

    let selected_vendor = match Select::with_theme(&theme())
        .with_prompt(format!(
            "Select a vendor: {}",
            "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
        ))
        .default(0)
        .items(&vendors)
        .max_length(config.items_per_page)
        .interact()
    {
        Ok(selected_index) => Vendors::vendor_normalize(vendors[selected_index]),

        Err(_) => {
            print_formatted_error("Creation failed. Failed to get a vendor.");
            std::process::exit(1);
        }
    };

    let mut secret_fields = Vec::new();
    let mut field_names = Vec::new();

    loop {
        let field_name = match Input::with_theme(&theme())
            .with_prompt("Type a field name:")
            .validate_with(|input: &String| -> Result<(), &str> {
                validate_secret_field_name(&input, "", &field_names)
            })
            .interact()
        {
            Ok(name) => {
                field_names.push(name.clone());
                name
            }

            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a field name.");
                std::process::exit(1);
            }
        };

        let field_value = match Password::with_theme(&theme())
            .with_prompt("Type a field value:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().chars().count() > 0 {
                    Ok(())
                } else {
                    Err("You are trying to set an empty value.")
                }
            })
            .interact()
        {
            Ok(value) => value,
            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a field value.");
                std::process::exit(1);
            }
        };

        secret_fields.push(create_secret::CreateSecretFieldInput {
            name: field_name.clone(),
            value: field_value,
        });

        let is_confirmed = match Confirm::new()
            .with_prompt("Do you want to add another field?")
            .interact()
        {
            Ok(confirmation) => confirmation,
            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a confirmation.");
                std::process::exit(1);
            }
        };

        if !is_confirmed {
            break;
        }
    }

    let secret_id =
        execute_graphql_request::<create_secret::Variables, create_secret::ResponseData>(
            headers.clone(),
            CreateSecret::build_query,
            &client,
            "Failed to create a secret",
            create_secret::Variables {
                fields: secret_fields,
                secret: create_secret::CreateSecretInput {
                    name: secret_name.clone(),
                    slug: slugify_prompt(&secret_name, "Type a slug for the secret:"),
                    project_id: project_id.to_string(),
                    vendor: selected_vendor.to_string(),
                },
            },
        )
        .create_secret
        .id;

    let text_template = minimad::TextTemplate::from(
        r#"
        ##### **✔ Secret successfully created**
        Secret link: ${secret-link}
        Secret ID: **${secret-id}**
        "#,
    );

    let mut expander = text_template.expander();

    let secret_link = style(format!(
        "{}/projects/{}/secrets/{}",
        config.webapp_url,
        &project_id.to_string().replace("-", ""),
        &secret_id.replace("-", "")
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    expander
        .set("secret-link", &secret_link)
        .set("secret-id", &secret_id);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Green);
    skin.print_expander(expander);
}
