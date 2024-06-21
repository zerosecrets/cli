#![allow(clippy::all, warnings)]
pub struct ProjectInfo;
pub mod project_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectInfo";
    pub const QUERY : & str = "query ProjectInfo($projectId: uuid!) {\n    project_by_pk(id: $projectId) {\n        id\n        name\n    }\n}\n" ;
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
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub project_by_pk: Option<ProjectInfoProjectByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectInfoProjectByPk {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for ProjectInfo {
    type Variables = project_info::Variables;
    type ResponseData = project_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_info::QUERY,
            operation_name: project_info::OPERATION_NAME,
        }
    }
}
