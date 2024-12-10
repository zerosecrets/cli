#![allow(clippy::all, warnings)]
pub struct UpdateSecretFields;
pub mod update_secret_fields {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateSecretFields";
    pub const QUERY : & str = "mutation UpdateSecretFields(\n  $id: String!\n  $name: String!\n  $userSecretFields: [UpdateUserSecretFieldsInput!]!\n) {\n  updateSecret(fields: $userSecretFields, id: $id, name: $name) {\n    success\n  }\n}\n" ;
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
    pub struct UpdateUserSecretFieldsInput {
        #[serde(rename = "decryptedValue")]
        pub decrypted_value: String,
        pub id: Option<String>,
        pub name: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
        pub name: String,
        #[serde(rename = "userSecretFields")]
        pub user_secret_fields: Vec<UpdateUserSecretFieldsInput>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "updateSecret")]
        pub update_secret: UpdateSecretFieldsUpdateSecret,
    }
    #[derive(Deserialize)]
    pub struct UpdateSecretFieldsUpdateSecret {
        pub success: Boolean,
    }
}
impl graphql_client::GraphQLQuery for UpdateSecretFields {
    type Variables = update_secret_fields::Variables;
    type ResponseData = update_secret_fields::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_secret_fields::QUERY,
            operation_name: update_secret_fields::OPERATION_NAME,
        }
    }
}
