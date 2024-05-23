mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
};
use crate::projects::usage::details::graphql::usage_detail::{usage_details, UsageDetails};
use clap::Args;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsUsageDetailsArgs {
    #[clap(
        short,
        long,
        help = "Usage ID (First 4 characters or more are allowed)"
    )]
    id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn usage_details(args: &ProjectsUsageDetailsArgs) {
    let config = Config::new();

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let usage_history_id = query_full_id(QueryType::UsageHistory, args.id.clone(), &access_token);

    let usage_details_error_message = format!(
        "Failed to get usage details with ID '{}'.",
        &usage_history_id
    );

    let usage_details_response =
        match execute_graphql_request::<usage_details::Variables, usage_details::ResponseData>(
            authorization_headers(&access_token),
            UsageDetails::build_query,
            &Client::new(),
            &usage_details_error_message,
            usage_details::Variables {
                id: usage_history_id,
            },
        )
        .usage_history_by_pk
        {
            Some(data) => data,

            None => {
                print_formatted_error(&format!(
                    "Usage details with ID '{}' not found.",
                    &usage_history_id.clone().to_string().dark_cyan()
                ));
                std::process::exit(1);
            }
        };

    let skin: MadSkin = MadSkin {
        ..Default::default()
    };

    let webapp_url = config.webapp_url;

    let mut chapters: Vec<String> = vec![
        format!(
            "**Record ID**: {}",
            usage_details_response.id.to_string().green()
        ),
        format!(
            "**Caller**   : {}",
            usage_details_response.caller_name.unwrap_or_else(|| "N\\A".to_string())
        ),
        format!("**Caller IP**: {}", usage_details_response.remote_ip.unwrap_or_else(|| "N\\A".to_string())),
        format!(
            "**Date**     : {}",
            format!(
                "{}",
                usage_details_response
                    .created_at
                    .format(&config.date_format)
                    .to_string()
            )
        ),
        format!(
            "**Project**  : {} ({})",
            usage_details_response.project.name,
            format!(
                "{}",
                style(format!(
                    "{}/projects/{}",
                    webapp_url,
                    usage_details_response.project.id.to_string().replace("-", "")
                ))
                .with(Color::Rgb {
                    r: 0,
                    g: 135,
                    b: 255,
                })
            )
        ),
        "**Secrets fetched**:".to_string(),
    ];

    let usage_secrets: &Vec<_> = &usage_details_response
        .secrets
        .iter()
        .map(|usage| &usage.user_secret)
        .collect();

    let mut secrets_vector = Vec::new();

    for usage_secret in usage_secrets {
        if let Some(secret) = usage_secret {
            let name_string = match secret.vendor {
                usage_details::vendorEnum_enum::agora => "Agora",
                usage_details::vendorEnum_enum::aws => "AWS",
                usage_details::vendorEnum_enum::azure => "Azure",
                usage_details::vendorEnum_enum::braintree => "Braintree",
                usage_details::vendorEnum_enum::digitalOcean => "DigitalOcean",
                usage_details::vendorEnum_enum::googleCloud => "GoogleCloud",
                usage_details::vendorEnum_enum::mailchimp => "Mailchimp",
                usage_details::vendorEnum_enum::mixpanel => "Mixpanel",
                usage_details::vendorEnum_enum::paypal => "Paypal",
                usage_details::vendorEnum_enum::pulumi => "Pulumi",
                usage_details::vendorEnum_enum::segment => "Segment",
                usage_details::vendorEnum_enum::sendgrid => "Sendgrid",
                usage_details::vendorEnum_enum::stripe => "Stripe",
                usage_details::vendorEnum_enum::terraform => "Terraform",
                usage_details::vendorEnum_enum::twilio => "Twilio",
                usage_details::vendorEnum_enum::ansible => "Ansible",
                usage_details::vendorEnum_enum::bitbucket => "Bitbucket",
                usage_details::vendorEnum_enum::claude => "Claude",
                usage_details::vendorEnum_enum::datadog => "Datadog",
                usage_details::vendorEnum_enum::docker => "Docker",
                usage_details::vendorEnum_enum::facebook => "Facebook",
                usage_details::vendorEnum_enum::gemini => "Gemini",
                usage_details::vendorEnum_enum::gitHub => "GitHub",
                usage_details::vendorEnum_enum::gitLab => "GitLab",
                usage_details::vendorEnum_enum::google => "Google",
                usage_details::vendorEnum_enum::jenkins => "Jenkins",
                usage_details::vendorEnum_enum::jira => "Jira",
                usage_details::vendorEnum_enum::kubernetes => "Kubernetes",
                usage_details::vendorEnum_enum::linear => "Linear",
                usage_details::vendorEnum_enum::openAI => "OpenAI",
                usage_details::vendorEnum_enum::salesforce => "Salesforce",
                usage_details::vendorEnum_enum::shopify => "Shopify",
                usage_details::vendorEnum_enum::slack => "Slack",
                usage_details::vendorEnum_enum::trello => "Trello",
                usage_details::vendorEnum_enum::zoom => "Zoom",
                usage_details::vendorEnum_enum::other => "Other",
                usage_details::vendorEnum_enum::Other(ref string) => string,
            };

            secrets_vector.push(format!(
                "  * {} ({})",
                name_string,
                style(format!(
                    "{}/projects/{}/secrets/{}",
                    webapp_url,
                    usage_details_response.project.id.to_string().replace("-", ""),
                    secret.id.to_string().replace("-", "")
                ))
                .with(Color::Rgb {
                    r: 0,
                    g: 135,
                    b: 255,
                })
            ));
        }
    }

    chapters.push(secrets_vector.join("\n"));
    skin.print_text(&chapters.join("\n"));
}
