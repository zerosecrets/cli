#![allow(clippy::all, warnings)]
pub struct GenerateSecretSharingUrl;
pub mod generate_secret_sharing_url {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GenerateSecretSharingUrl";
    pub const QUERY : & str = "mutation GenerateSecretSharingUrl(\n  $expiresAt: String!\n  $passphrase: String!\n  $secretsFieldIds: [String!]!\n) {\n  generateSecretSharingUrl(\n    expiresAt: $expiresAt\n    passphrase: $passphrase\n    secretsFieldIds: $secretsFieldIds\n  ) {\n    url\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "expiresAt")]
        pub expires_at: String,
        pub passphrase: String,
        #[serde(rename = "secretsFieldIds")]
        pub secrets_field_ids: Vec<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "generateSecretSharingUrl")]
        pub generate_secret_sharing_url: GenerateSecretSharingUrlGenerateSecretSharingUrl,
    }
    #[derive(Deserialize)]
    pub struct GenerateSecretSharingUrlGenerateSecretSharingUrl {
        pub url: String,
    }
}
impl graphql_client::GraphQLQuery for GenerateSecretSharingUrl {
    type Variables = generate_secret_sharing_url::Variables;
    type ResponseData = generate_secret_sharing_url::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: generate_secret_sharing_url::QUERY,
            operation_name: generate_secret_sharing_url::OPERATION_NAME,
        }
    }
}
