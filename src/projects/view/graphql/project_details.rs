#![allow(clippy::all, warnings)]
pub struct ProjectDetails;
pub mod project_details {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectDetails";
    pub const QUERY : & str = "query ProjectDetails($id: uuid!) {\n  project(where: {id: {_eq: $id}}) {\n    id\n    name\n    description\n\n    owner {\n      id\n      name\n      email\n    }\n\n    usageHistories(limit: 1, order_by: {createdAt: desc}) {\n      createdAt\n    }\n\n    teams_aggregate {\n      aggregate {\n        count\n      }\n    }\n\n    integrationInstallations_aggregate {\n      aggregate {\n        count\n      }\n    }\n\n    userSecrets_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
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
        pub owner: ProjectDetailsProjectOwner,
        #[serde(rename = "usageHistories")]
        pub usage_histories: Vec<ProjectDetailsProjectUsageHistories>,
        pub teams_aggregate: ProjectDetailsProjectTeamsAggregate,
        #[serde(rename = "integrationInstallations_aggregate")]
        pub integration_installations_aggregate:
            ProjectDetailsProjectIntegrationInstallationsAggregate,
        #[serde(rename = "userSecrets_aggregate")]
        pub user_secrets_aggregate: ProjectDetailsProjectUserSecretsAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectOwner {
        pub id: uuid,
        pub name: String,
        pub email: String,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectUsageHistories {
        #[serde(rename = "createdAt")]
        pub created_at: timestamptz,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectTeamsAggregate {
        pub aggregate: Option<ProjectDetailsProjectTeamsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsProjectTeamsAggregateAggregate {
        pub count: Int,
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
