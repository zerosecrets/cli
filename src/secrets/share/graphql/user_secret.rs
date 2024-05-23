#![allow(clippy::all, warnings)]
pub struct UserSecret;
pub mod user_secret {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UserSecret";
    pub const QUERY : & str = "query UserSecret($id: uuid!) {\n  userSecret_by_pk(id: $id) {\n    id\n    name\n    projectId\n\n    fields {\n      id\n      name\n    }\n  }\n}\n" ;
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
        #[serde(rename = "userSecret_by_pk")]
        pub user_secret_by_pk: Option<UserSecretUserSecretByPk>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretUserSecretByPk {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "projectId")]
        pub project_id: uuid,
        pub fields: Vec<UserSecretUserSecretByPkFields>,
    }
    #[derive(Deserialize)]
    pub struct UserSecretUserSecretByPkFields {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserSecret {
    type Variables = user_secret::Variables;
    type ResponseData = user_secret::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_secret::QUERY,
            operation_name: user_secret::OPERATION_NAME,
        }
    }
}
