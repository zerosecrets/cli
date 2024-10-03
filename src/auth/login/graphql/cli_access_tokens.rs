#![allow(clippy::all, warnings)]
pub struct CliAccessTokens;
pub mod cli_access_tokens {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CliAccessTokens";
    pub const QUERY : & str = "mutation CliAccessTokens($object: CliAccessTokensInput!) {\n  cliAccessTokens(object: $object) {\n    accessToken\n    userId\n  }\n}\n" ;
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
    pub struct CliAccessTokensInput {
        pub code: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub object: CliAccessTokensInput,
    }
    impl Variables {
        pub fn new(code: String) -> Self {
            let object = CliAccessTokensInput { code };
            Self { object }
        }
    }
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "cliAccessTokens")]
        pub cli_access_tokens: CliAccessTokensCliAccessTokens,
    }
    #[derive(Deserialize)]
    pub struct CliAccessTokensCliAccessTokens {
        #[serde(rename = "accessToken")]
        pub access_token: String,
        #[serde(rename = "userId")]
        pub user_id: String,
    }
}
impl graphql_client::GraphQLQuery for CliAccessTokens {
    type Variables = cli_access_tokens::Variables;
    type ResponseData = cli_access_tokens::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: cli_access_tokens::QUERY,
            operation_name: cli_access_tokens::OPERATION_NAME,
        }
    }
}
