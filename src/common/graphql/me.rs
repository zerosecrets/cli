#![allow(clippy::all, warnings)]
pub struct Me;
pub mod me {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "Me";
    pub const QUERY: &str = "query Me {\n  me {\n    id\n  }\n}\n";
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = uuid::Uuid;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub me: MeMe,
    }
    #[derive(Deserialize)]
    pub struct MeMe {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for Me {
    type Variables = me::Variables;
    type ResponseData = me::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: me::QUERY,
            operation_name: me::OPERATION_NAME,
        }
    }
}
