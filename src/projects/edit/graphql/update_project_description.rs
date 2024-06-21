#![allow(clippy::all, warnings)]
pub struct UpdateProjectDescription;
pub mod update_project_description {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateProjectDescription";
    pub const QUERY : & str = "mutation UpdateProjectDescription($projectId: uuid!, $projectDescription: String!) {\n    update_project(where: {id: {_eq: $projectId}}, _set: {description: $projectDescription}) {\n        affected_rows\n    }\n}\n" ;
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
    pub struct Variables {
        #[serde(rename = "projectId")]
        pub project_id: uuid,
        #[serde(rename = "projectDescription")]
        pub project_description: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update_project: Option<UpdateProjectDescriptionUpdateProject>,
    }
    #[derive(Deserialize)]
    pub struct UpdateProjectDescriptionUpdateProject {
        pub affected_rows: Int,
    }
}
impl graphql_client::GraphQLQuery for UpdateProjectDescription {
    type Variables = update_project_description::Variables;
    type ResponseData = update_project_description::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_project_description::QUERY,
            operation_name: update_project_description::OPERATION_NAME,
        }
    }
}
