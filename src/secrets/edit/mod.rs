mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
    slugify::slugify,
    validate_secret_field_name::validate_secret_field_name,
    validate_secret_name::validate_secret_name,
    vendors::Vendors,
};
use clap::Args;
use dialoguer::{Input, Password, Select};
use graphql::update_secret_fields::{update_secret_fields, UpdateSecretFields};
use graphql::update_secret_info::{update_secret_info, UpdateSecretInfo};
use graphql::user_secret_and_already_taken_user_secret_names::{
    user_secret_and_already_taken_user_secret_names, UserSecretAndAlreadyTakenUserSecretNames,
};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use strum::IntoEnumIterator;
use termimad::crossterm::style::{style, Color, Stylize};

#[derive(Args, Debug)]
pub struct SecretsEditArgs {
    #[clap(
        short,
        long,
        help = "Secret ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Secret field key, not required, if passed, it updates a specific field within the secret"
    )]
    key: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn edit(args: &SecretsEditArgs) {
    let config = Config::new();

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let secret_id = query_full_id(QueryType::UserSecret, args.id.clone(), &access_token);
    let client = Client::new();
    let headers = authorization_headers(&access_token);
    let query_secret_info_error_message = "Editing failed. Failed to retrieve the secret info.";

    let secret_info = match execute_graphql_request::<
        user_secret_and_already_taken_user_secret_names::Variables,
        user_secret_and_already_taken_user_secret_names::ResponseData,
    >(
        headers.clone(),
        UserSecretAndAlreadyTakenUserSecretNames::build_query,
        &client,
        query_secret_info_error_message,
        user_secret_and_already_taken_user_secret_names::Variables { id: secret_id },
    )
    .user_secret_by_pk
    {
        Some(data) => data,

        None => {
            print_formatted_error(query_secret_info_error_message);
            std::process::exit(1);
        }
    };

    // update the secret field if args.key is passed
    if let Some(key) = &args.key {
        // first check that the user is passing in the correct key and have default values
        let editable_secret_field = match secret_info
            .fields
            .iter()
            .find(|field| field.name == key.to_string())
        {
            Some(secret) => secret,

            None => {
                print_formatted_error(&format!(
                    "Editing failed. The key '{}' doesn't exist in the secret '{}'.",
                    &key, &secret_info.name
                ));
                std::process::exit(1);
            }
        };

        let existed_fields_keys: Vec<String> = secret_info
            .fields
            .iter()
            .map(|field| field.name.to_string())
            .collect();

        let new_field_key_name: String = match Input::with_theme(&theme())
            .with_prompt("Type a new field key:")
            .default(editable_secret_field.name.to_string())
            .validate_with(|input: &String| -> Result<(), &str> {
                return validate_secret_field_name(
                    &input,
                    &editable_secret_field.name,
                    &existed_fields_keys,
                );
            })
            .interact()
        {
            Ok(name) => name,

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the field key.");
                std::process::exit(1);
            }
        };

        let new_field_value = match Password::with_theme(&theme())
            .with_prompt("Type a new field value:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().chars().count() > 0 {
                    Ok(())
                } else {
                    print_formatted_error("Editing failed. You are trying to set an empty value.");
                    std::process::exit(1);
                }
            })
            .interact()
        {
            Ok(password) => password,

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the field value.");
                std::process::exit(1);
            }
        };

        let updated_user_secret_fields = secret_info
            .fields
            .iter()
            .map(|field: &user_secret_and_already_taken_user_secret_names::UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkFields| {
                if editable_secret_field.name == field.name {
                    update_secret_fields::UpdateUserSecretFieldsInput {
                        id: Some(field.id.to_string()),
                        name: new_field_key_name.to_string(),
                        decrypted_value: new_field_value.to_string()
                    }
                } else {
                    update_secret_fields::UpdateUserSecretFieldsInput {
                        id: Some(field.id.to_string()),
                        name: field.name.to_string(),
                        decrypted_value: field.value.to_string()
                    }
                }
            })
            .collect();

        let update_secret_field_error_message =
            "Editing failed. Failed to update the secret field.";

        let is_update_secret_field_success = execute_graphql_request::<
            update_secret_fields::Variables,
            update_secret_fields::ResponseData,
        >(
            headers.clone(),
            UpdateSecretFields::build_query,
            &client,
            update_secret_field_error_message,
            update_secret_fields::Variables {
                id: secret_info.id.to_string(),
                name: secret_info.name,
                user_secret_fields: updated_user_secret_fields,
            },
        )
        .update_secret
        .success;

        if !is_update_secret_field_success {
            print_formatted_error(update_secret_field_error_message);
            std::process::exit(1);
        }

    // otherwise update the secret information
    } else {
        let existed_secret_names: Vec<String> = match secret_info.project {
            Some(project) => project,

            None => {
                print_formatted_error("Editing failed. Failed to get existed secret names.");
                std::process::exit(1);
            }
        }
        .user_secrets
        .iter()
        .map(|secret| secret.name.to_string())
        .collect();

        let new_secret_name: String = match Input::with_theme(&theme())
            .with_prompt("Type a new secret name:")
            .default(secret_info.name.clone())
            .validate_with(|input: &String| -> Result<(), &str> {
                return validate_secret_name(&input, &secret_info.name, &existed_secret_names);
            })
            .interact()
        {
            Ok(name) => name,

            Err(_) => {
                print_formatted_error(&format!("Editing failed. Failed to get the secret name."));
                std::process::exit(1);
            }
        };

        let vendors: Vec<_> = update_secret_info::vendorEnum_enum::iter().collect();

        let vendors_variants: &Vec<String> =
            &vendors.iter().map(|variant| variant.to_string()).collect();

        let default_vendor = match vendors_variants
            .iter()
            .position(|variant| variant.to_string() == secret_info.vendor.to_string())
        {
            Some(position) => position,
            None => 0,
        };

        let new_secret_vendor = match Select::with_theme(&theme())
            .with_prompt(format!(
                "Select a vendor: {}",
                "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
            ))
            .items(&Vendors::new().prettified_vendor_options)
            .default(default_vendor)
            .max_length(config.items_per_page)
            .interact()
        {
            Ok(selected_index) => vendors[selected_index].clone(),

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the secret vendor.");
                std::process::exit(1);
            }
        };

        let update_secret_error_message = format!(
            "Editing failed. Failed to update the secret with ID '{}'.",
            args.id.clone()
        );

        // Don't forget add to generated query [serde(skip_serializing_if = "Option::is_none")] if you re-generate query
        execute_graphql_request::<update_secret_info::Variables, update_secret_info::ResponseData>(
            headers,
            UpdateSecretInfo::build_query,
            &client,
            &update_secret_error_message,
            update_secret_info::Variables {
                id: secret_info.id,
                set: update_secret_info::userSecret_set_input {
                    name: Some(new_secret_name.to_owned()),
                    vendor: Some(new_secret_vendor),
                    slug: Some(slugify(&new_secret_name)),
                },
            },
        )
        .update_user_secret_by_pk;
    }

    println!(
        "{} {}",
        "✔".green(),
        "The secret has been successfully updated."
    );

    println!(
        "{}",
        style(format!(
            "{}/projects/{}/secrets/{}",
            config.webapp_url,
            secret_info.project_id.to_string().replace("-", ""),
            secret_info.id.to_string().replace("-", "")
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
    );
}
