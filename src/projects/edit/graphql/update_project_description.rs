#![allow(clippy::all, warnings)]
pub struct UpdateProjectDescription;
pub mod update_project_description {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UpdateProjectDescription";
    pub const QUERY : & str = "mutation UpdateProjectDescription($projectId: uuid!, $userId: uuid!, $projectDescription: String!) {\n    update_token(where: {id: {_eq: $projectId}, ownerUserId: {_eq: $userId}}, _set: {description: $projectDescription}) {\n        affected_rows\n    }\n}\n" ;
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
        #[serde(rename = "projectDescription")]
        pub project_description: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update_token: Option<UpdateProjectDescriptionUpdateToken>,
    }
    #[derive(Deserialize)]
    pub struct UpdateProjectDescriptionUpdateToken {
        pub affected_rows: Int,
    }
}
impl graphql_client::GraphQLQuery for UpdateProjectDescription {
    type Variables = update_project_description::Variables;
    type ResponseData = update_project_description::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_project_description::QUERY,
            operation_name: update_project_description::OPERATION_NAME,
        }
    }
}
