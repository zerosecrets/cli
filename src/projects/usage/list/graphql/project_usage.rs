#![allow(clippy::all, warnings)]
pub struct ProjectUsage;
pub mod project_usage {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectUsage";
    pub const QUERY : & str = "query ProjectUsage($id: uuid!) {\n  token(where: {id: {_eq: $id}}, limit: 1000) {\n    id\n    name\n\n    usageHistory(order_by: {createdAt: desc}) {\n      id\n      createdAt\n      callerName\n      remoteIp\n\n      secrets_aggregate {\n        aggregate {\n          count\n        }\n      }\n    }\n\n    usageHistory_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
    use chrono::offset::Utc;
    use chrono::DateTime;
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
        pub token: Vec<ProjectUsageToken>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageToken {
        pub id: uuid::Uuid,
        pub name: String,
        #[serde(rename = "usageHistory")]
        pub usage_history: Vec<ProjectUsageTokenUsageHistory>,
        #[serde(rename = "usageHistory_aggregate")]
        pub usage_history_aggregate: ProjectUsageTokenUsageHistoryAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageTokenUsageHistory {
        pub id: uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime<Utc>,
        #[serde(rename = "callerName")]
        pub caller_name: Option<String>,
        #[serde(rename = "remoteIp")]
        pub remote_ip: String,
        pub secrets_aggregate: ProjectUsageTokenUsageHistorySecretsAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageTokenUsageHistorySecretsAggregate {
        pub aggregate: Option<ProjectUsageTokenUsageHistorySecretsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageTokenUsageHistorySecretsAggregateAggregate {
        pub count: Int,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageTokenUsageHistoryAggregate {
        pub aggregate: Option<ProjectUsageTokenUsageHistoryAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageTokenUsageHistoryAggregateAggregate {
        pub count: Int,
    }
}
impl graphql_client::GraphQLQuery for ProjectUsage {
    type Variables = project_usage::Variables;
    type ResponseData = project_usage::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_usage::QUERY,
            operation_name: project_usage::OPERATION_NAME,
        }
    }
}
