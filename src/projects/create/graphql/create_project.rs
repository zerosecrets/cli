#![allow(clippy::all, warnings)]
pub struct CreateProject;
pub mod create_project {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CreateProject";
    pub const QUERY : & str = "mutation CreateProject($object: CreateProjectInput!) {\n    createProject(object: $object) {\n        id\n        token\n    }\n}\n" ;
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
        #[serde(rename = "activeTeamId")]
        pub active_team_id: uuid::Uuid,
        pub icon: String,
        pub name: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub object: CreateProjectInput,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createProject")]
        pub create_project: Option<CreateProjectCreateProject>,
    }
    #[derive(Deserialize)]
    pub struct CreateProjectCreateProject {
        pub id: uuid::Uuid,
        pub token: String,
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
