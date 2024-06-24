#![allow(clippy::all, warnings)]
pub struct DeleteProject;
pub mod delete_project {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteProject";
    pub const QUERY : & str = "mutation DeleteProject($id: uuid!) {\n    delete_project_by_pk(id: $id) {\n      id\n    }\n}\n" ;
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
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub delete_project_by_pk: Option<DeleteProjectDeleteProjectByPk>,
    }
    #[derive(Deserialize)]
    pub struct DeleteProjectDeleteProjectByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for DeleteProject {
    type Variables = delete_project::Variables;
    type ResponseData = delete_project::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_project::QUERY,
            operation_name: delete_project::OPERATION_NAME,
        }
    }
}
