mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
    read_env_file::read_env_file,
    write_env_file::write_env_file,
};
use clap::Args;
use graphql::secret_fields_ids::{secret_fields_ids, SecretFieldsIds};
use graphql::view_secret::{view_secret, ViewSecret};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct SecretDropArgs {
    #[clap(
        short,
        long,
        help = "Secret ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(short, long, help = "Path to the env file")]
    env_file: String,
    #[clap(
        short,
        long,
        help = "Field key to drop (If not specified, all fields will be dropped)"
    )]
    field_key: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn drop(args: &SecretDropArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let user_secret_id = query_full_id(QueryType::UserSecret, args.id.clone(), &access_token);
    let user_secret_fields_error_message =
        format!("Dump failed. Failed to retrieve the user's secret fields.");

    let user_secret_fields: Vec<secret_fields_ids::SecretFieldsIdsUserSecretByPkFields> = match execute_graphql_request::<
        secret_fields_ids::Variables,
        secret_fields_ids::ResponseData,
    >(
        authorization_headers(&access_token),
        SecretFieldsIds::build_query,
        &Client::new(),
        &user_secret_fields_error_message,
        secret_fields_ids::Variables { id: user_secret_id },
    )
    .user_secret_by_pk
    {
        Some(data) => data.fields,

        None => {
            print_formatted_error(&user_secret_fields_error_message);
            std::process::exit(1);
        }
    };

    if user_secret_fields.len() == 0 {
        print_formatted_error("Dump failed. The user's secret has no fields.");
        std::process::exit(1);
    }

    let mut new_fields = read_env_file(&args.env_file);

    fn get_field_value(access_token: &str, field_id: &str) -> String {
        execute_graphql_request::<view_secret::Variables, view_secret::ResponseData>(
            authorization_headers(access_token),
            ViewSecret::build_query,
            &Client::new(),
            "Dump failed. Failed to retrieve the user's secret field value.",
            view_secret::Variables {
                secret_field: field_id.to_string(),
            },
        )
        .view_secret
        .value
    }

    // If the field key is specified, only drop that field.
    if let Some(field_key) = &args.field_key {
        let user_secret_field = user_secret_fields
            .iter()
            .find(|field| &field.name == field_key)
            .unwrap_or_else(|| {
                print_formatted_error(&format!(
                    "Dump failed. The '{}' field was not found.",
                    field_key
                ));

                std::process::exit(1);
            });

        let field_value = get_field_value(&access_token, &user_secret_field.id.to_string());
        new_fields.insert(user_secret_field.name.clone(), field_value);

    // If the field key is not specified, drop all fields.
    } else {
        user_secret_fields.iter().for_each(|field| {
            let field_value = get_field_value(&access_token, &field.id.to_string());
            new_fields.insert(field.name.clone(), field_value);
        });
    };

    user_secret_fields.iter().for_each(|field| {
        if let Some(field_key) = &args.field_key {
            if &field.name == field_key {
                let field_value = get_field_value(&access_token, &field.id.to_string());
                new_fields.insert(field.name.clone(), field_value);
            }
        } else {
            let field_value = get_field_value(&access_token, &field.id.to_string());
            new_fields.insert(field.name.clone(), field_value);
        }
    });

    write_env_file(&args.env_file, &new_fields);
    println!("{} {}", "✓".green(), "Secret fields dropped successfully.");
}
