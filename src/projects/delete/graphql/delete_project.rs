#![allow(clippy::all, warnings)]
pub struct DeleteProject;
pub mod delete_project {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "DeleteProject";
    pub const QUERY : & str = "mutation DeleteProject($object: DeleteProjectsInput!) {\n    deleteProjects(object: $object) {\n        affected_rows\n    }\n}\n" ;
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
    pub struct DeleteProjectsInput {
        #[serde(rename = "deleteSecretsOn3rdParty")]
        pub delete_secrets_on3rd_party: Option<Boolean>,
        pub ids: Vec<uuid::Uuid>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub object: DeleteProjectsInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "deleteProjects")]
        pub delete_projects: Option<DeleteProjectDeleteProjects>,
    }
    #[derive(Deserialize)]
    pub struct DeleteProjectDeleteProjects {
        pub affected_rows: Int,
    }
}
impl graphql_client::GraphQLQuery for DeleteProject {
    type Variables = delete_project::Variables;
    type ResponseData = delete_project::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_project::QUERY,
            operation_name: delete_project::OPERATION_NAME,
        }
    }
}
