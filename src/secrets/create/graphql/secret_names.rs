#![allow(clippy::all, warnings)]
pub struct SecretNames;
pub mod secret_names {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SecretNames";
    pub const QUERY : & str = "query SecretNames($projectId: uuid!) {\n    token_by_pk(id: $projectId) {\n        userSecret {\n            id\n            name\n        }\n    }\n}\n" ;
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
        #[serde(rename = "projectId")]
        pub project_id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub token_by_pk: Option<SecretNamesTokenByPk>,
    }
    #[derive(Deserialize)]
    pub struct SecretNamesTokenByPk {
        #[serde(rename = "userSecret")]
        pub user_secret: Vec<SecretNamesTokenByPkUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct SecretNamesTokenByPkUserSecret {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SecretNames {
    type Variables = secret_names::Variables;
    type ResponseData = secret_names::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: secret_names::QUERY,
            operation_name: secret_names::OPERATION_NAME,
        }
    }
}
