#![allow(clippy::all, warnings)]
pub struct GenerateSecretSharingUrl;
pub mod generate_secret_sharing_url {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "GenerateSecretSharingUrl";
    pub const QUERY : & str = "mutation GenerateSecretSharingUrl($object: GenerateSecretSharingUrlInput!) {\n    generateSecretSharingUrl(object: $object) {\n        url\n    }\n}\n" ;

    use chrono::{DateTime, Utc};
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
    pub struct GenerateSecretSharingUrlInput {
        #[serde(rename = "expiresAt")]
        pub expires_at: DateTime<Utc>,
        #[serde(rename = "passPhrase")]
        pub pass_phrase: Option<String>,
        #[serde(rename = "secretsFieldsIds")]
        pub secrets_fields_ids: Vec<String>,
        #[serde(rename = "tokenId")]
        pub token_id: uuid::Uuid,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub object: GenerateSecretSharingUrlInput,
    }
    impl Variables {
        pub fn new(
            expires_at: DateTime<Utc>,
            pass_phrase: Option<String>,
            secrets_fields_ids: Vec<String>,
            token_id: uuid::Uuid,
        ) -> Self {
            let object = GenerateSecretSharingUrlInput {
                expires_at,
                pass_phrase,
                secrets_fields_ids,
                token_id,
            };
            Self { object }
        }
    }
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
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: generate_secret_sharing_url::QUERY,
            operation_name: generate_secret_sharing_url::OPERATION_NAME,
        }
    }
}
