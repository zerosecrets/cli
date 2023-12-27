#![allow(clippy::all, warnings)]
pub struct SearchUsageHistoryById;
pub mod search_usage_history_by_id {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SearchUsageHistoryById";
    pub const QUERY : & str = "query SearchUsageHistoryById($id: String!) {\n  search_usage_history_by_id(args: {match: $id}) {\n    id\n    createdAt\n  }\n}\n" ;
    use chrono::{DateTime, Utc};
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
        pub id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search_usage_history_by_id: Vec<SearchUsageHistoryByIdSearchUsageHistoryById>,
    }
    #[derive(Deserialize)]
    pub struct SearchUsageHistoryByIdSearchUsageHistoryById {
        pub id: uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime<Utc>,
    }
}
impl graphql_client::GraphQLQuery for SearchUsageHistoryById {
    type Variables = search_usage_history_by_id::Variables;
    type ResponseData = search_usage_history_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: search_usage_history_by_id::QUERY,
            operation_name: search_usage_history_by_id::OPERATION_NAME,
        }
    }
}
