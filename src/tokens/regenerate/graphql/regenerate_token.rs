#![allow(clippy::all, warnings)]
pub struct RegenerateToken;
pub mod regenerate_token {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RegenerateToken";
    pub const QUERY : & str = "mutation RegenerateToken($id: ID!, $expiresAt: String) {\n  regenerateProjectToken(id: $id, expiresAt: $expiresAt) {\n    value\n  }\n}\n" ;
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
        pub id: ID,
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "regenerateProjectToken")]
        pub regenerate_project_token: RegenerateTokenRegenerateProjectToken,
    }
    #[derive(Deserialize)]
    pub struct RegenerateTokenRegenerateProjectToken {
        pub value: String,
    }
}
impl graphql_client::GraphQLQuery for RegenerateToken {
    type Variables = regenerate_token::Variables;
    type ResponseData = regenerate_token::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: regenerate_token::QUERY,
            operation_name: regenerate_token::OPERATION_NAME,
        }
    }
}
