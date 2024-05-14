#![allow(clippy::all, warnings)]
pub struct TokenList;
pub mod token_list {
    #![allow(dead_code)]
    use chrono::DateTime;
    use chrono::offset::Utc;
    pub const OPERATION_NAME: &str = "TokenList";
    pub const QUERY : & str = "query TokenList($id: uuid!) {\n  project(where: {id: {_eq: $id}}) {\n    id\n    name\n\n    tokens {\n      id\n      name\n      expiresAt\n    }\n  }\n}\n" ;
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
        pub project: Vec<TokenListProject>,
    }
    #[derive(Deserialize)]
    pub struct TokenListProject {
        pub id: uuid,
        pub name: String,
        pub tokens: Vec<TokenListProjectTokens>,
    }
    #[derive(Deserialize)]
    pub struct TokenListProjectTokens {
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
