#![allow(clippy::all, warnings)]
pub struct ProjectDetails;
pub mod project_details {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectDetails";
    pub const QUERY : & str = "query ProjectDetails($id: uuid!) {\n  project(where: {id: {_eq: $id}}) {\n    id\n    name\n    description\n\n    usageHistories(limit: 1, order_by: {createdAt: desc}) {\n      createdAt\n    }\n\n    team {\n      name\n    }\n\n    integrationInstallations_aggregate {\n      aggregate {\n        count\n      }\n    }\n\n    userSecrets_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use ::uuid::Uuid;
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
    type timestamptz = DateTime<Utc>;
    type uuid = Uuid;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub project: Vec<ProjectDetailsProject>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProject {
        pub id: uuid,
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "usageHistories")]
        pub usage_histories: Vec<ProjectDetailsProjectUsageHistories>,
        pub team: Option<ProjectDetailsProjectTeam>,
        #[serde(rename = "integrationInstallations_aggregate")]
        pub integration_installations_aggregate:
            ProjectDetailsProjectIntegrationInstallationsAggregate,
        #[serde(rename = "userSecrets_aggregate")]
        pub user_secrets_aggregate: ProjectDetailsProjectUserSecretsAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectUsageHistories {
        #[serde(rename = "createdAt")]
        pub created_at: timestamptz,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectTeam {
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectIntegrationInstallationsAggregate {
        pub aggregate: Option<ProjectDetailsProjectIntegrationInstallationsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectIntegrationInstallationsAggregateAggregate {
        pub count: Int,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectUserSecretsAggregate {
        pub aggregate: Option<ProjectDetailsProjectUserSecretsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectUserSecretsAggregateAggregate {
        pub count: Int,
    }
}
impl graphql_client::GraphQLQuery for ProjectDetails {
    type Variables = project_details::Variables;
    type ResponseData = project_details::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_details::QUERY,
            operation_name: project_details::OPERATION_NAME,
        }
    }
}
