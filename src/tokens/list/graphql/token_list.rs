#![allow(clippy::all, warnings)]
pub struct TokenList;
pub mod token_list {
    #![allow(dead_code)]
    use std::result::Result;
    use chrono::DateTime;
    use chrono::offset::Utc;
    pub const OPERATION_NAME: &str = "TokenList";
    pub const QUERY : & str = "query TokenList($id: uuid!) {\n  token(where: {id: {_eq: $id}}) {\n    id\n    name\n\n    project_projectTokens {\n      id\n      name\n      expiresAt\n    }\n  }\n}\n" ;
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

    type timestamptz = DateTime<Utc>;
    type uuid = ::uuid::Uuid;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub token: Vec<TokenListToken>,
    }
    #[derive(Deserialize)]
    pub struct TokenListToken {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "project_projectTokens")]
        pub project_project_tokens: Vec<TokenListTokenProjectProjectTokens>,
    }
    #[derive(Deserialize)]
    pub struct TokenListTokenProjectProjectTokens {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<timestamptz>,
    }
}
impl graphql_client::GraphQLQuery for TokenList {
    type Variables = token_list::Variables;
    type ResponseData = token_list::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: token_list::QUERY,
            operation_name: token_list::OPERATION_NAME,
        }
    }
}
