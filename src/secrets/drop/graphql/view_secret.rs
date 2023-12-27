#![allow(clippy::all, warnings)]
pub struct ViewSecret;
pub mod view_secret {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ViewSecret";
    pub const QUERY : & str = "query ViewSecret($secretField: String!) {\n    viewSecret(secretField: {secretFieldId: $secretField}) {\n        value\n    }\n}\n" ;
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
        #[serde(rename = "secretField")]
        pub secret_field: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "viewSecret")]
        pub view_secret: ViewSecretViewSecret,
    }
    #[derive(Deserialize)]
    pub struct ViewSecretViewSecret {
        pub value: String,
    }
}
impl graphql_client::GraphQLQuery for ViewSecret {
    type Variables = view_secret::Variables;
    type ResponseData = view_secret::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: view_secret::QUERY,
            operation_name: view_secret::OPERATION_NAME,
        }
    }
}
