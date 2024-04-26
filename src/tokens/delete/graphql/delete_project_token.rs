#![allow(clippy::all, warnings)]
pub struct DeleteToken;
pub mod delete_token {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteToken";
    pub const QUERY : & str = "mutation DeleteToken($tokenId: uuid!) {\n    delete_projectToken_by_pk(id: $tokenId) {\n        id\n    }\n}\n" ;
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
        #[serde(rename = "tokenId")]
        pub token_id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "delete_projectToken_by_pk")]
        pub delete_project_token_by_pk: Option<DeleteTokenDeleteProjectTokenByPk>,
    }
    #[derive(Deserialize)]
    pub struct DeleteTokenDeleteProjectTokenByPk {
        pub id: uuid::Uuid,
    }
}
impl graphql_client::GraphQLQuery for DeleteToken {
    type Variables = delete_token::Variables;
    type ResponseData = delete_token::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_token::QUERY,
            operation_name: delete_token::OPERATION_NAME,
        }
    }
}
