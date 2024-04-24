mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    pad_to_column_width::pad_to_column_width,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
    table::table,
};
use crate::projects::tokens::list::graphql::token_list::{token_list, TokenList};
use clap::Args;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct ProjectsTokenListArgs {
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

pub fn list(args: &ProjectsTokenListArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_id = query_full_id(QueryType::Project, args.id.clone(), &access_token);

    let project_tokens_error_message = format!(
        "Failed to retrieve tokens for the project with ID '{}'.",
        &args.id
    );

    let project_token_list_response =
        execute_graphql_request::<token_list::Variables, token_list::ResponseData>(
            authorization_headers(&access_token),
            TokenList::build_query,
            &Client::new(),
            &project_tokens_error_message,
            token_list::Variables { id: project_id },
        )
        .token;

    let project_token_list = match project_token_list_response.first() {
        Some(tokens) => tokens,
        None => {
            print_formatted_error(&project_tokens_error_message);
            std::process::exit(1);
        }
    };

    struct ColumnWidthSize {
        id: usize,
        name: usize,
        expires: usize,
    }

    let mut column_width_size = ColumnWidthSize {
        id: 5,
        name: 0,
        expires: 10,
    };

    let date_format = &Config::new().date_format;

    // save the length of the longest column element
    for token in &project_token_list.project_project_tokens {
        let expires_at_data = match &token.expires_at {
            Some(date) => date.format(date_format).to_string(),
            None => "Infinity".to_string(),
        };

        column_width_size.name = column_width_size.name.max(token.name.len());
        column_width_size.expires = column_width_size.expires.max(expires_at_data.len());
    }

    let mut list = Vec::new();
    let indentation = 2;

    for token in &project_token_list.project_project_tokens {
        let expires_at_data = match &token.expires_at {
            Some(date) => date.format(date_format).to_string(),
            None => "Infinity".to_string(),
        };

        list.push(format!(
            "{}{}{}",
            pad_to_column_width(
                format!("#{}", &token.id.to_string()[0..4]),
                column_width_size.id + indentation
            )
            .green(),
            pad_to_column_width(token.name.to_string(), column_width_size.name + indentation),
            pad_to_column_width(expires_at_data, column_width_size.expires + indentation),
        ));
    }

    table(
        list,
        format!("Tokens of project named '{}'", project_token_list.name),
        "You don't have any token for this project.",
        format!(
            "{}{}{}",
            pad_to_column_width("ID".to_string(), column_width_size.id + indentation),
            pad_to_column_width("NAME".to_string(), column_width_size.name + indentation),
            pad_to_column_width(
                "EXPIRES".to_string(),
                column_width_size.expires + indentation
            ),
        ),
    )
}
