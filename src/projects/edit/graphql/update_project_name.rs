#![allow(clippy::all, warnings)]
pub struct UpdateProjectName;
pub mod update_project_name {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateProjectName";
    pub const QUERY : & str = "mutation UpdateProjectName($id: uuid!, $name: String!, $slug: String!) {\n  update_project(\n    where: { id: { _eq: $id } }\n    _set: { name: $name, slug: $slug }\n  ) {\n    affected_rows\n  }\n}\n" ;
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
        pub id: uuid,
        pub name: String,
        pub slug: String,
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
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_project_name::QUERY,
            operation_name: update_project_name::OPERATION_NAME,
        }
    }
}
