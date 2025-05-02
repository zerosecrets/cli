#![allow(clippy::all, warnings)]
pub struct UserSecretDetails;
pub mod user_secret_details {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserSecretDetails";
    pub const QUERY : & str = "query UserSecretDetails($id: uuid!) {\n  userSecret_by_pk(id: $id) {\n    id\n    name\n    vendor\n    updatedAt\n    projectId\n\n    fields {\n      id\n      name\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    use chrono::offset::Utc;
    use chrono::DateTime;
    type ID = String;
    type timestamptz = DateTime<Utc>;
    type uuid = ::uuid::Uuid;
    #[derive()]
    pub enum secretsVaultVendor_enum {
        agora,
        amazonwebservices,
        ansible,
        auth0,
        bitbucket,
        braintree,
        cloudflare,
        datadog,
        digitalocean,
        discord,
        docker,
        dropbox,
        facebook,
        figma,
        github,
        gitlab,
        google,
        googlecloud,
        googlegemini,
        jenkins,
        jirasoftware,
        kubernetes,
        linear,
        linkedin,
        mailchimp,
        microsoftazure,
        mixpanel,
        netlify,
        openai,
        other,
        paypal,
        pulumi,
        reddit,
        salesforce,
        segment,
        sendgrid,
        shopify,
        slack,
        stripe,
        terraform,
        trello,
        twilio,
        vercel,
        x,
        zoom,
        Other(String),
    }
    impl ::serde::Serialize for secretsVaultVendor_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                secretsVaultVendor_enum::agora => "agora",
                secretsVaultVendor_enum::amazonwebservices => "amazonwebservices",
                secretsVaultVendor_enum::ansible => "ansible",
                secretsVaultVendor_enum::auth0 => "auth0",
                secretsVaultVendor_enum::bitbucket => "bitbucket",
                secretsVaultVendor_enum::braintree => "braintree",
                secretsVaultVendor_enum::cloudflare => "cloudflare",
                secretsVaultVendor_enum::datadog => "datadog",
                secretsVaultVendor_enum::digitalocean => "digitalocean",
                secretsVaultVendor_enum::discord => "discord",
                secretsVaultVendor_enum::docker => "docker",
                secretsVaultVendor_enum::dropbox => "dropbox",
                secretsVaultVendor_enum::facebook => "facebook",
                secretsVaultVendor_enum::figma => "figma",
                secretsVaultVendor_enum::github => "github",
                secretsVaultVendor_enum::gitlab => "gitlab",
                secretsVaultVendor_enum::google => "google",
                secretsVaultVendor_enum::googlecloud => "googlecloud",
                secretsVaultVendor_enum::googlegemini => "googlegemini",
                secretsVaultVendor_enum::jenkins => "jenkins",
                secretsVaultVendor_enum::jirasoftware => "jirasoftware",
                secretsVaultVendor_enum::kubernetes => "kubernetes",
                secretsVaultVendor_enum::linear => "linear",
                secretsVaultVendor_enum::linkedin => "linkedin",
                secretsVaultVendor_enum::mailchimp => "mailchimp",
                secretsVaultVendor_enum::microsoftazure => "microsoftazure",
                secretsVaultVendor_enum::mixpanel => "mixpanel",
                secretsVaultVendor_enum::netlify => "netlify",
                secretsVaultVendor_enum::openai => "openai",
                secretsVaultVendor_enum::other => "other",
                secretsVaultVendor_enum::paypal => "paypal",
                secretsVaultVendor_enum::pulumi => "pulumi",
                secretsVaultVendor_enum::reddit => "reddit",
                secretsVaultVendor_enum::salesforce => "salesforce",
                secretsVaultVendor_enum::segment => "segment",
                secretsVaultVendor_enum::sendgrid => "sendgrid",
                secretsVaultVendor_enum::shopify => "shopify",
                secretsVaultVendor_enum::slack => "slack",
                secretsVaultVendor_enum::stripe => "stripe",
                secretsVaultVendor_enum::terraform => "terraform",
                secretsVaultVendor_enum::trello => "trello",
                secretsVaultVendor_enum::twilio => "twilio",
                secretsVaultVendor_enum::vercel => "vercel",
                secretsVaultVendor_enum::x => "x",
                secretsVaultVendor_enum::zoom => "zoom",
                secretsVaultVendor_enum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for secretsVaultVendor_enum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "agora" => Ok(secretsVaultVendor_enum::agora),
                "amazonwebservices" => Ok(secretsVaultVendor_enum::amazonwebservices),
                "ansible" => Ok(secretsVaultVendor_enum::ansible),
                "auth0" => Ok(secretsVaultVendor_enum::auth0),
                "bitbucket" => Ok(secretsVaultVendor_enum::bitbucket),
                "braintree" => Ok(secretsVaultVendor_enum::braintree),
                "cloudflare" => Ok(secretsVaultVendor_enum::cloudflare),
                "datadog" => Ok(secretsVaultVendor_enum::datadog),
                "digitalocean" => Ok(secretsVaultVendor_enum::digitalocean),
                "discord" => Ok(secretsVaultVendor_enum::discord),
                "docker" => Ok(secretsVaultVendor_enum::docker),
                "dropbox" => Ok(secretsVaultVendor_enum::dropbox),
                "facebook" => Ok(secretsVaultVendor_enum::facebook),
                "figma" => Ok(secretsVaultVendor_enum::figma),
                "github" => Ok(secretsVaultVendor_enum::github),
                "gitlab" => Ok(secretsVaultVendor_enum::gitlab),
                "google" => Ok(secretsVaultVendor_enum::google),
                "googlecloud" => Ok(secretsVaultVendor_enum::googlecloud),
                "googlegemini" => Ok(secretsVaultVendor_enum::googlegemini),
                "jenkins" => Ok(secretsVaultVendor_enum::jenkins),
                "jirasoftware" => Ok(secretsVaultVendor_enum::jirasoftware),
                "kubernetes" => Ok(secretsVaultVendor_enum::kubernetes),
                "linear" => Ok(secretsVaultVendor_enum::linear),
                "linkedin" => Ok(secretsVaultVendor_enum::linkedin),
                "mailchimp" => Ok(secretsVaultVendor_enum::mailchimp),
                "microsoftazure" => Ok(secretsVaultVendor_enum::microsoftazure),
                "mixpanel" => Ok(secretsVaultVendor_enum::mixpanel),
                "netlify" => Ok(secretsVaultVendor_enum::netlify),
                "openai" => Ok(secretsVaultVendor_enum::openai),
                "other" => Ok(secretsVaultVendor_enum::other),
                "paypal" => Ok(secretsVaultVendor_enum::paypal),
                "pulumi" => Ok(secretsVaultVendor_enum::pulumi),
                "reddit" => Ok(secretsVaultVendor_enum::reddit),
                "salesforce" => Ok(secretsVaultVendor_enum::salesforce),
                "segment" => Ok(secretsVaultVendor_enum::segment),
                "sendgrid" => Ok(secretsVaultVendor_enum::sendgrid),
                "shopify" => Ok(secretsVaultVendor_enum::shopify),
                "slack" => Ok(secretsVaultVendor_enum::slack),
                "stripe" => Ok(secretsVaultVendor_enum::stripe),
                "terraform" => Ok(secretsVaultVendor_enum::terraform),
                "trello" => Ok(secretsVaultVendor_enum::trello),
                "twilio" => Ok(secretsVaultVendor_enum::twilio),
                "vercel" => Ok(secretsVaultVendor_enum::vercel),
                "x" => Ok(secretsVaultVendor_enum::x),
                "zoom" => Ok(secretsVaultVendor_enum::zoom),
                _ => Ok(secretsVaultVendor_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "userSecret_by_pk")]
        pub user_secret_by_pk: Option<UserSecretDetailsUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretDetailsUserSecretByPk {
        pub id: uuid,
        pub name: String,
        pub vendor: secretsVaultVendor_enum,
        #[serde(rename = "updatedAt")]
        pub updated_at: timestamptz,
        #[serde(rename = "projectId")]
        pub project_id: uuid,
        pub fields: Vec<UserSecretDetailsUserSecretByPkFields>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretDetailsUserSecretByPkFields {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserSecretDetails {
    type Variables = user_secret_details::Variables;
    type ResponseData = user_secret_details::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_secret_details::QUERY,
            operation_name: user_secret_details::OPERATION_NAME,
        }
    }
}
