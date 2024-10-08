#![allow(clippy::all, warnings)]
pub struct CreateSecret;
pub mod create_secret {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CreateSecret";
    pub const QUERY : & str = "mutation CreateSecret($fields: [CreateSecretFieldInput!]!, $secret: CreateSecretInput!, $userId: String!) {\n    createSecret(fields: $fields, secret: $secret, userId: $userId) {\n        secretId\n    }\n}\n" ;
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
    pub struct CreateSecretFieldInput {
        pub name: String,
        pub value: String,
    }
    #[derive(Serialize)]
    pub struct CreateSecretInput {
        pub name: String,
        #[serde(rename = "projectId")]
        pub project_id: String,
        pub vendor: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub fields: Vec<CreateSecretFieldInput>,
        pub secret: CreateSecretInput,
        #[serde(rename = "userId")]
        pub user_id: String,
    }
    impl Variables {
        pub fn new(
            name: String,
            project_id: String,
            vendor: String,
            fields: Vec<CreateSecretFieldInput>,
            user_id: String,
        ) -> Self {
            let secret = CreateSecretInput {
                name,
                project_id,
                vendor,
            };

            Self {
                secret,
                fields,
                user_id,
            }
        }
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createSecret")]
        pub create_secret: CreateSecretCreateSecret,
    }
    #[derive(Deserialize)]
    pub struct CreateSecretCreateSecret {
        #[serde(rename = "secretId")]
        pub secret_id: String,
    }
}
impl graphql_client::GraphQLQuery for CreateSecret {
    type Variables = create_secret::Variables;
    type ResponseData = create_secret::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_secret::QUERY,
            operation_name: create_secret::OPERATION_NAME,
        }
    }
}
