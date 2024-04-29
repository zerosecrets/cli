#![allow(clippy::all, warnings)]
pub struct SearchTokenById;
pub mod search_token_by_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SearchTokenById";
    pub const QUERY : & str = "query SearchTokenById($id: String!) {\n  search_token_by_id(args: {match: $id}) {\n    id\n    name\n  }\n}\n" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search_token_by_id: Vec<SearchTokenByIdSearchTokenById>,
    }
    #[derive(Deserialize)]
    pub struct SearchTokenByIdSearchTokenById {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SearchTokenById {
    type Variables = search_token_by_id::Variables;
    type ResponseData = search_token_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search_token_by_id::QUERY,
            operation_name: search_token_by_id::OPERATION_NAME,
        }
    }
}
