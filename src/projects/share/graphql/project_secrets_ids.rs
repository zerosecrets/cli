#![allow(clippy::all, warnings)]
pub struct ProjectSecretsIds;
pub mod project_secrets_ids {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectSecretsIds";
    pub const QUERY : & str = "query ProjectSecretsIds($projectId: uuid!) {\n  token_by_pk(id: $projectId) {\n    id\n\n    userSecret{\n      id\n      name\n\n      fields {\n        id\n      }\n    }\n  }\n}\n" ;
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
    pub struct Variables {
        #[serde(rename = "projectId")]
        pub project_id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub token_by_pk: Option<ProjectSecretsIdsTokenByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsTokenByPk {
        pub id: uuid,
        #[serde(rename = "userSecret")]
        pub user_secret: Vec<ProjectSecretsIdsTokenByPkUserSecret>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsTokenByPkUserSecret {
        pub id: uuid,
        pub name: String,
        pub fields: Vec<ProjectSecretsIdsTokenByPkUserSecretFields>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsTokenByPkUserSecretFields {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for ProjectSecretsIds {
    type Variables = project_secrets_ids::Variables;
    type ResponseData = project_secrets_ids::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_secrets_ids::QUERY,
            operation_name: project_secrets_ids::OPERATION_NAME,
        }
    }
}
