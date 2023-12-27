#![allow(clippy::all, warnings)]
pub struct SecretFieldsIds;
pub mod secret_fields_ids {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SecretFieldsIds";
    pub const QUERY : & str = "query SecretFieldsIds($id: uuid!) {\n   userSecret_by_pk(id: $id) {\n    id\n\n    fields {\n      id\n      name\n    }\n  }\n}\n" ;
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
        pub id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "userSecret_by_pk")]
        pub user_secret_by_pk: Option<SecretFieldsIdsUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct SecretFieldsIdsUserSecretByPk {
        pub id: uuid::Uuid,
        pub fields: Vec<SecretFieldsIdsUserSecretByPkFields>,
    }
    #[derive(Deserialize)]
    pub struct SecretFieldsIdsUserSecretByPkFields {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SecretFieldsIds {
    type Variables = secret_fields_ids::Variables;
    type ResponseData = secret_fields_ids::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: secret_fields_ids::QUERY,
            operation_name: secret_fields_ids::OPERATION_NAME,
        }
    }
}
