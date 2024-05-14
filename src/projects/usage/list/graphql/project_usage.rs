#![allow(clippy::all, warnings)]
pub struct ProjectUsage;
pub mod project_usage {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectUsage";
    pub const QUERY : & str = "query ProjectUsage($id: uuid!) {\n  project(where: {id: {_eq: $id}}, limit: 1000) {\n    id\n    name\n\n    usageHistories(order_by: {createdAt: desc}) {\n      id\n      createdAt\n      callerName\n      remoteIp\n\n      secrets_aggregate {\n        aggregate {\n          count\n        }\n      }\n    }\n\n    usageHistories_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
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
        pub project: Vec<ProjectUsageProject>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProject {
        pub id: uuid::Uuid,
        pub name: String,
        #[serde(rename = "usageHistories")]
        pub usage_histories: Vec<ProjectUsageProjectUsageHistories>,
        #[serde(rename = "usageHistories_aggregate")]
        pub usage_histories_aggregate: ProjectUsageProjectUsageHistoriesAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProjectUsageHistories {
        pub id: uuid::Uuid,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime<Utc>,
        #[serde(rename = "callerName")]
        pub caller_name: Option<String>,
        #[serde(rename = "remoteIp")]
        pub remote_ip: Option<String>,
        pub secrets_aggregate: ProjectUsageProjectUsageHistoriesSecretsAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProjectUsageHistoriesSecretsAggregate {
        pub aggregate: Option<ProjectUsageProjectUsageHistoriesSecretsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProjectUsageHistoriesSecretsAggregateAggregate {
        pub count: Int,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProjectUsageHistoriesAggregate {
        pub aggregate: Option<ProjectUsageProjectUsageHistoriesAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectUsageProjectUsageHistoriesAggregateAggregate {
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
