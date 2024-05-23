#![allow(clippy::all, warnings)]
pub struct ViewSecret;
pub mod view_secret {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ViewSecret";
    pub const QUERY : & str = "query ViewSecret($secretId: String!) {\n    viewSecretFields(secretField: {secretId: $secretId}) {\n        secretFields {\n            key\n            value\n        }\n    }\n}\n" ;
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
        #[serde(rename = "secretId")]
        pub secret_id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "viewSecretFields")]
        pub view_secret_fields: ViewSecretViewSecretFields,
    }
    #[derive(Deserialize)]
    pub struct ViewSecretViewSecretFields {
        #[serde(rename = "secretFields")]
        pub secret_fields: Vec<ViewSecretViewSecretFieldsSecretFields>,
    }
    #[derive(Deserialize)]
    pub struct ViewSecretViewSecretFieldsSecretFields {
        pub key: String,
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
