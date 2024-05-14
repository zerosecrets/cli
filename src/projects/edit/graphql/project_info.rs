#![allow(clippy::all, warnings)]
pub struct ProjectInfo;
pub mod project_info {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectInfo";
    pub const QUERY : & str = "query ProjectInfo($projectId: uuid!) {\n    project_by_pk(id: $projectId) {\n        id\n        name\n        ownerUserId\n    }\n}\n" ;
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
        pub project_by_pk: Option<ProjectInfoProjectByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectInfoProjectByPk {
        pub id: uuid::Uuid,
        pub name: String,
        #[serde(rename = "ownerUserId")]
        pub owner_user_id: uuid::Uuid,
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
