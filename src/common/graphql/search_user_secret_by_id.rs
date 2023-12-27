#![allow(clippy::all, warnings)]
pub struct SearchUserSecretById;
pub mod search_user_secret_by_id {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SearchUserSecretById";
    pub const QUERY : & str = "query SearchUserSecretById($id: String!)  {\n  search_user_secret_by_id(args: {match: $id}) {\n    id\n    name\n  }\n}\n" ;
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
        pub search_user_secret_by_id: Vec<SearchUserSecretByIdSearchUserSecretById>,
    }
    #[derive(Deserialize)]
    pub struct SearchUserSecretByIdSearchUserSecretById {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SearchUserSecretById {
    type Variables = search_user_secret_by_id::Variables;
    type ResponseData = search_user_secret_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search_user_secret_by_id::QUERY,
            operation_name: search_user_secret_by_id::OPERATION_NAME,
        }
    }
}
