#![allow(clippy::all, warnings)]
pub struct SearchUserById;
pub mod search_user_by_id {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SearchUserById";
    pub const QUERY : & str = "query SearchUserById($id: String!)  {\n  search_user_by_id(args: {match: $id}) {\n    id\n    email\n    name\n  }\n}\n" ;
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
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search_user_by_id: Vec<SearchUserByIdSearchUserById>,
    }
    #[derive(Deserialize)]
    pub struct SearchUserByIdSearchUserById {
        pub id: uuid,
        pub email: String,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SearchUserById {
    type Variables = search_user_by_id::Variables;
    type ResponseData = search_user_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search_user_by_id::QUERY,
            operation_name: search_user_by_id::OPERATION_NAME,
        }
    }
}
