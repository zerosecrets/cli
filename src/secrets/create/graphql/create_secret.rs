#![allow(clippy::all, warnings)]
pub struct CreateSecret;
pub mod create_secret {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateSecret";
    pub const QUERY : & str = "mutation CreateSecret(\n  $fields: [CreateSecretFieldInput!]!\n  $secret: CreateSecretInput!\n) {\n  createSecret(fields: $fields, secret: $secret) {\n    id\n\n    userSecret {\n      id\n      slug\n    }\n  }\n}\n" ;
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
    type uuid = ::uuid::Uuid;
    #[derive(Serialize)]
    pub struct CreateSecretFieldInput {
        pub name: String,
        pub value: String,
    }
    #[derive(Serialize)]
    pub struct CreateSecretInput {
        pub name: String,
        #[serde(rename = "projectId")]
        pub project_id: ID,
        pub vendor: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub fields: Vec<CreateSecretFieldInput>,
        pub secret: CreateSecretInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createSecret")]
        pub create_secret: CreateSecretCreateSecret,
    }
    #[derive(Deserialize)]
    pub struct CreateSecretCreateSecret {
        pub id: ID,
        #[serde(rename = "userSecret")]
        pub user_secret: Option<CreateSecretCreateSecretUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct CreateSecretCreateSecretUserSecret {
        pub id: uuid,
        pub slug: String,
    }
}
impl graphql_client::GraphQLQuery for CreateSecret {
    type Variables = create_secret::Variables;
    type ResponseData = create_secret::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_secret::QUERY,
            operation_name: create_secret::OPERATION_NAME,
        }
    }
}
