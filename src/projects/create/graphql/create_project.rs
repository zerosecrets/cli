#![allow(clippy::all, warnings)]
pub struct CreateProject;
pub mod create_project {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CreateProject";
    pub const QUERY : & str = "mutation CreateProject($object: CreateProjectInput!) {\n    createProject(object: $object) {\n        projectId\n        tokenValue\n    }\n}\n" ;
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
    pub struct CreateProjectInput {
        #[serde(rename = "projectIcon")]
        pub project_icon: String,
        #[serde(rename = "projectName")]
        pub project_name: String,
        pub token: Option<TokenObject>,
    }
    #[derive(Serialize)]
    pub struct TokenObject {
        #[serde(rename = "tokenExpiresAt")]
        pub token_expires_at: Option<String>,
        #[serde(rename = "tokenName")]
        pub token_name: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub object: CreateProjectInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createProject")]
        pub create_project: CreateProjectCreateProject,
    }
    #[derive(Deserialize)]
    pub struct CreateProjectCreateProject {
        #[serde(rename = "projectId")]
        pub project_id: Option<String>,
        #[serde(rename = "tokenValue")]
        pub token_value: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for CreateProject {
    type Variables = create_project::Variables;
    type ResponseData = create_project::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_project::QUERY,
            operation_name: create_project::OPERATION_NAME,
        }
    }
}
