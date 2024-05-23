#![allow(clippy::all, warnings)]
pub struct UpdateProjectName;
pub mod update_project_name {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UpdateProjectName";
    pub const QUERY : & str = "mutation UpdateProjectName($projectId: uuid!, $userId: uuid!, $projectName: String!) {\n    update_project(where: {id: {_eq: $projectId}, ownerUserId: {_eq: $userId}}, _set: {name: $projectName}) {\n        affected_rows\n    }\n}\n" ;
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
        #[serde(rename = "userId")]
        pub user_id: uuid::Uuid,
        #[serde(rename = "projectName")]
        pub project_name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update_project: Option<UpdateProjectNameUpdateProject>,
    }
    #[derive(Deserialize)]
    pub struct UpdateProjectNameUpdateProject {
        pub affected_rows: Int,
    }
}
impl graphql_client::GraphQLQuery for UpdateProjectName {
    type Variables = update_project_name::Variables;
    type ResponseData = update_project_name::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_project_name::QUERY,
            operation_name: update_project_name::OPERATION_NAME,
        }
    }
}
