#![allow(clippy::all, warnings)]
pub struct DeleteUserSecret;
pub mod delete_user_secret {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "DeleteUserSecret";
    pub const QUERY : & str = "mutation DeleteUserSecret($id: uuid!) {\n    delete_userSecret_by_pk(id: $id) {\n        id\n    }\n}\n" ;
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
        #[serde(rename = "delete_userSecret_by_pk")]
        pub delete_user_secret_by_pk: Option<DeleteUserSecretDeleteUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct DeleteUserSecretDeleteUserSecretByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for DeleteUserSecret {
    type Variables = delete_user_secret::Variables;
    type ResponseData = delete_user_secret::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_user_secret::QUERY,
            operation_name: delete_user_secret::OPERATION_NAME,
        }
    }
}
