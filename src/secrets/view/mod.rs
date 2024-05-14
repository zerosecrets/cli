mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    format_relative_time::format_relative_time,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use clap::Args;
use graphql::user_secret_details::{user_secret_details, UserSecretDetails};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    MadSkin,
};

#[derive(Args, Debug)]
pub struct SecretViewArgs {
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

pub fn view(args: &SecretViewArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let user_secret_id = query_full_id(QueryType::UserSecret, args.id.clone(), &access_token);

    let user_secret_details_error_message = format!(
        "Failed to retrieve user secret details with ID '{}'.",
        &user_secret_id.to_string()[..4]
    );

    let user_secret_details = match execute_graphql_request::<
        user_secret_details::Variables,
        user_secret_details::ResponseData,
    >(
        authorization_headers(&access_token),
        UserSecretDetails::build_query,
        &Client::new(),
        &user_secret_details_error_message,
        user_secret_details::Variables { id: user_secret_id },
    )
    .user_secret_by_pk
    {
        Some(data) => data,

        None => {
            print_formatted_error(&user_secret_details_error_message);
            std::process::exit(1);
        }
    };

    let vendor = match user_secret_details.vendor {
        user_secret_details::vendorEnum_enum::Other(_) => "Other",
        user_secret_details::vendorEnum_enum::other => "Other",
        user_secret_details::vendorEnum_enum::agora => "Agora",
        user_secret_details::vendorEnum_enum::aws => "AWS",
        user_secret_details::vendorEnum_enum::azure => "Azure",
        user_secret_details::vendorEnum_enum::braintree => "Braintree",
        user_secret_details::vendorEnum_enum::digitalOcean => "Digital ocean",
        user_secret_details::vendorEnum_enum::googleCloud => "Google cloud",
        user_secret_details::vendorEnum_enum::mailchimp => "Mailchimp",
        user_secret_details::vendorEnum_enum::mixpanel => "Mixpanel",
        user_secret_details::vendorEnum_enum::paypal => "PayPal",
        user_secret_details::vendorEnum_enum::pulumi => "Pulumi",
        user_secret_details::vendorEnum_enum::segment => "Segment",
        user_secret_details::vendorEnum_enum::sendgrid => "Sendgrid",
        user_secret_details::vendorEnum_enum::stripe => "Stripe",
        user_secret_details::vendorEnum_enum::terraform => "Terraform",
        user_secret_details::vendorEnum_enum::twilio => "Twilio",
        user_secret_details::vendorEnum_enum::zoom => "Zoom",
        user_secret_details::vendorEnum_enum::claude => "Claude",
        user_secret_details::vendorEnum_enum::datadog => "Datadog",
        user_secret_details::vendorEnum_enum::docker => "Docker",
        user_secret_details::vendorEnum_enum::facebook => "Facebook",
        user_secret_details::vendorEnum_enum::gemini => "Gemini",
        user_secret_details::vendorEnum_enum::gitHub => "GitHub",
        user_secret_details::vendorEnum_enum::gitLab => "GitLab",
        user_secret_details::vendorEnum_enum::google => "Google",
        user_secret_details::vendorEnum_enum::jenkins => "Jenkins",
        user_secret_details::vendorEnum_enum::jira => "Jira",
        user_secret_details::vendorEnum_enum::kubernetes => "Kubernetes",
        user_secret_details::vendorEnum_enum::linear => "Linear",
        user_secret_details::vendorEnum_enum::shopify => "Shopify",
        user_secret_details::vendorEnum_enum::slack => "Slack",
        user_secret_details::vendorEnum_enum::trello => "Trello",
        user_secret_details::vendorEnum_enum::ansible => "Ansible",
        user_secret_details::vendorEnum_enum::bitbucket => "Bitbucket",
        user_secret_details::vendorEnum_enum::openAI => "OpenAI",
        user_secret_details::vendorEnum_enum::salesforce => "Salesforce",
    };

    let last_usage = match format_relative_time(&user_secret_details.updated_at.to_string()) {
        Ok(relative_time) => relative_time,

        Err(_) => {
            print_formatted_error("Failed to format the last usage time.");
            std::process::exit(1);
        }
    };

    let mut fields = Vec::new();

    for field in user_secret_details.fields {
        fields.push(format!("  {}\n", field.name));
    }

    let skin = MadSkin {
        ..Default::default()
    };

    let markdown_text = format!(
        "**URL**: {} \n**Name**: {} \n**Vendor**: {}\n**Last used**: {}\n**Fields**:\n{}",
        style(format!(
            "{}/projects/{}/secrets/{}",
            Config::new().webapp_url,
            user_secret_details.project_id.to_string().replace("-", ""),
            user_secret_details.id.to_string().replace("-", "")
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        }),
        &user_secret_details.name,
        &vendor,
        &last_usage,
        &fields.join(""),
    );

    skin.print_text(&markdown_text);
}
