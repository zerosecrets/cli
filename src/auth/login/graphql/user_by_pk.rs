#![allow(clippy::all, warnings)]
pub struct UserByPk;
pub mod user_by_pk {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UserByPk";
    pub const QUERY: &str =
        "query UserByPk($id: uuid!) {\n  user_by_pk(id: $id) {\n    id\n    name\n  }\n}\n";
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
        pub user_by_pk: Option<UserByPkUserByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserByPkUserByPk {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserByPk {
    type Variables = user_by_pk::Variables;
    type ResponseData = user_by_pk::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_by_pk::QUERY,
            operation_name: user_by_pk::OPERATION_NAME,
        }
    }
}
