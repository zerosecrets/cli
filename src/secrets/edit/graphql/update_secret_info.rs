#![allow(clippy::all, warnings)]
pub struct UpdateSecretInfo;
pub mod update_secret_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateSecretInfo";
    pub const QUERY : & str = "mutation UpdateSecretInfo($id: uuid!, $set: userSecret_set_input!) {\n  update_userSecret_by_pk(pk_columns: { id: $id }, _set: $set) {\n    id\n  }\n}\n" ;
    use super::*;
    use strum_macros::{Display, EnumIter};
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type uuid = ::uuid::Uuid;
    #[derive(EnumIter, Display, Clone)]
    pub enum vendorEnum_enum {
        agora,
        ansible,
        aws,
        azure,
        bitbucket,
        braintree,
        cloudflare,
        datadog,
        digitalOcean,
        docker,
        facebook,
        gemini,
        gitHub,
        gitLab,
        google,
        googleCloud,
        jenkins,
        jira,
        kubernetes,
        linear,
        mailchimp,
        mixpanel,
        netlify,
        openAI,
        other,
        paypal,
        pulumi,
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
        zoom,
        Other(String),
    }
    impl ::serde::Serialize for vendorEnum_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                vendorEnum_enum::agora => "agora",
                vendorEnum_enum::ansible => "ansible",
                vendorEnum_enum::aws => "aws",
                vendorEnum_enum::azure => "azure",
                vendorEnum_enum::bitbucket => "bitbucket",
                vendorEnum_enum::braintree => "braintree",
                vendorEnum_enum::cloudflare => "cloudflare",
                vendorEnum_enum::datadog => "datadog",
                vendorEnum_enum::digitalOcean => "digitalOcean",
                vendorEnum_enum::docker => "docker",
                vendorEnum_enum::facebook => "facebook",
                vendorEnum_enum::gemini => "gemini",
                vendorEnum_enum::gitHub => "gitHub",
                vendorEnum_enum::gitLab => "gitLab",
                vendorEnum_enum::google => "google",
                vendorEnum_enum::googleCloud => "googleCloud",
                vendorEnum_enum::jenkins => "jenkins",
                vendorEnum_enum::jira => "jira",
                vendorEnum_enum::kubernetes => "kubernetes",
                vendorEnum_enum::linear => "linear",
                vendorEnum_enum::mailchimp => "mailchimp",
                vendorEnum_enum::mixpanel => "mixpanel",
                vendorEnum_enum::netlify => "netlify",
                vendorEnum_enum::openAI => "openAI",
                vendorEnum_enum::other => "other",
                vendorEnum_enum::paypal => "paypal",
                vendorEnum_enum::pulumi => "pulumi",
                vendorEnum_enum::salesforce => "salesforce",
                vendorEnum_enum::segment => "segment",
                vendorEnum_enum::sendgrid => "sendgrid",
                vendorEnum_enum::shopify => "shopify",
                vendorEnum_enum::slack => "slack",
                vendorEnum_enum::stripe => "stripe",
                vendorEnum_enum::terraform => "terraform",
                vendorEnum_enum::trello => "trello",
                vendorEnum_enum::twilio => "twilio",
                vendorEnum_enum::vercel => "vercel",
                vendorEnum_enum::zoom => "zoom",
                vendorEnum_enum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for vendorEnum_enum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "agora" => Ok(vendorEnum_enum::agora),
                "ansible" => Ok(vendorEnum_enum::ansible),
                "aws" => Ok(vendorEnum_enum::aws),
                "azure" => Ok(vendorEnum_enum::azure),
                "bitbucket" => Ok(vendorEnum_enum::bitbucket),
                "braintree" => Ok(vendorEnum_enum::braintree),
                "cloudflare" => Ok(vendorEnum_enum::cloudflare),
                "datadog" => Ok(vendorEnum_enum::datadog),
                "digitalOcean" => Ok(vendorEnum_enum::digitalOcean),
                "docker" => Ok(vendorEnum_enum::docker),
                "facebook" => Ok(vendorEnum_enum::facebook),
                "gemini" => Ok(vendorEnum_enum::gemini),
                "gitHub" => Ok(vendorEnum_enum::gitHub),
                "gitLab" => Ok(vendorEnum_enum::gitLab),
                "google" => Ok(vendorEnum_enum::google),
                "googleCloud" => Ok(vendorEnum_enum::googleCloud),
                "jenkins" => Ok(vendorEnum_enum::jenkins),
                "jira" => Ok(vendorEnum_enum::jira),
                "kubernetes" => Ok(vendorEnum_enum::kubernetes),
                "linear" => Ok(vendorEnum_enum::linear),
                "mailchimp" => Ok(vendorEnum_enum::mailchimp),
                "mixpanel" => Ok(vendorEnum_enum::mixpanel),
                "netlify" => Ok(vendorEnum_enum::netlify),
                "openAI" => Ok(vendorEnum_enum::openAI),
                "other" => Ok(vendorEnum_enum::other),
                "paypal" => Ok(vendorEnum_enum::paypal),
                "pulumi" => Ok(vendorEnum_enum::pulumi),
                "salesforce" => Ok(vendorEnum_enum::salesforce),
                "segment" => Ok(vendorEnum_enum::segment),
                "sendgrid" => Ok(vendorEnum_enum::sendgrid),
                "shopify" => Ok(vendorEnum_enum::shopify),
                "slack" => Ok(vendorEnum_enum::slack),
                "stripe" => Ok(vendorEnum_enum::stripe),
                "terraform" => Ok(vendorEnum_enum::terraform),
                "trello" => Ok(vendorEnum_enum::trello),
                "twilio" => Ok(vendorEnum_enum::twilio),
                "vercel" => Ok(vendorEnum_enum::vercel),
                "zoom" => Ok(vendorEnum_enum::zoom),
                _ => Ok(vendorEnum_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct userSecret_set_input {
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub note: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub slug: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vendor: Option<vendorEnum_enum>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
        pub set: userSecret_set_input,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "update_userSecret_by_pk")]
        pub update_user_secret_by_pk: Option<UpdateSecretInfoUpdateUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UpdateSecretInfoUpdateUserSecretByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for UpdateSecretInfo {
    type Variables = update_secret_info::Variables;
    type ResponseData = update_secret_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_secret_info::QUERY,
            operation_name: update_secret_info::OPERATION_NAME,
        }
    }
}
