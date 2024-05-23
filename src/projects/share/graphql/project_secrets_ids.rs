#![allow(clippy::all, warnings)]
pub struct ProjectSecretsIds;
pub mod project_secrets_ids {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectSecretsIds";
    pub const QUERY : & str = "query ProjectSecretsIds($projectId: uuid!) {\n  project_by_pk(id: $projectId) {\n    id\n\n    userSecrets{\n      id\n      name\n\n      fields {\n        id\n      }\n    }\n  }\n}\n" ;
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
        pub project_by_pk: Option<ProjectSecretsIdsProjectByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsProjectByPk {
        pub id: uuid,
        #[serde(rename = "userSecrets")]
        pub user_secrets: Vec<ProjectSecretsIdsProjectByPkUserSecrets>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsProjectByPkUserSecrets {
        pub id: uuid,
        pub name: String,
        pub fields: Vec<ProjectSecretsIdsProjectByPkUserSecretsFields>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsIdsProjectByPkUserSecretsFields {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for ProjectSecretsIds {
    type Variables = project_secrets_ids::Variables;
    type ResponseData = project_secrets_ids::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_secrets_ids::QUERY,
            operation_name: project_secrets_ids::OPERATION_NAME,
        }
    }
}
