#![allow(clippy::all, warnings)]
pub struct SearchProjectById;
pub mod search_project_by_id {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SearchProjectById";
    pub const QUERY : & str = "query SearchProjectById($id: String!) {\n  search_project_by_id(args: {match: $id}) {\n    id\n    name\n  }\n}\n" ;
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
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search_project_by_id: Vec<SearchProjectByIdSearchProjectById>,
    }
    #[derive(Deserialize)]
    pub struct SearchProjectByIdSearchProjectById {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SearchProjectById {
    type Variables = search_project_by_id::Variables;
    type ResponseData = search_project_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search_project_by_id::QUERY,
            operation_name: search_project_by_id::OPERATION_NAME,
        }
    }
}
