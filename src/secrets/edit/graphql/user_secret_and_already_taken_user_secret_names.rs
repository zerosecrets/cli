#![allow(clippy::all, warnings)]
pub struct UserSecretAndAlreadyTakenUserSecretNames;
pub mod user_secret_and_already_taken_user_secret_names {
    #![allow(dead_code)]
    use std::result::Result;
    use strum_macros::{Display, EnumIter};
    pub const OPERATION_NAME: &str = "UserSecretAndAlreadyTakenUserSecretNames";
    pub const QUERY : & str = "query UserSecretAndAlreadyTakenUserSecretNames($id: uuid!) {\n  userSecret_by_pk(id: $id) {\n    id\n    name\n    slug\n    vendor\n    projectId\n\n    fields {\n      id\n      name\n      value\n      slug\n    }\n\n    project {\n      id\n\n      userSecrets {\n        id\n        name\n      }\n    }\n  }\n}\n" ;
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
    #[derive(EnumIter, Display)]
    pub enum vendorEnum_enum {
        agora,
        ansible,
        aws,
        azure,
        bitbucket,
        braintree,
        claude,
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
                vendorEnum_enum::claude => "claude",
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
                "claude" => Ok(vendorEnum_enum::claude),
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
                "zoom" => Ok(vendorEnum_enum::zoom),
                _ => Ok(vendorEnum_enum::Other(s)),
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
        pub user_secret_by_pk: Option<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPk {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        pub vendor: vendorEnum_enum,
        #[serde(rename = "projectId")]
        pub project_id: uuid,
        pub fields: Vec<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkFields>,
        pub project: Option<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkProject>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkFields {
        pub id: uuid,
        pub name: String,
        pub value: String,
        pub slug: String,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkProject {
        pub id: uuid,
        #[serde(rename = "userSecrets")]
        pub user_secrets:
            Vec<UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkProjectUserSecrets>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretAndAlreadyTakenUserSecretNamesUserSecretByPkProjectUserSecrets {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserSecretAndAlreadyTakenUserSecretNames {
    type Variables = user_secret_and_already_taken_user_secret_names::Variables;
    type ResponseData = user_secret_and_already_taken_user_secret_names::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_secret_and_already_taken_user_secret_names::QUERY,
            operation_name: user_secret_and_already_taken_user_secret_names::OPERATION_NAME,
        }
    }
}
