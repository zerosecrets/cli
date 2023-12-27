#![allow(clippy::all, warnings)]
pub struct ProjectDetails;
pub mod project_details {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "ProjectDetails";
    pub const QUERY : & str = "query ProjectDetails($id: uuid!) {\n  token(where: {id: {_eq: $id}}) {\n    id\n    name\n    description\n\n    owner {\n      id\n      name\n      email\n    }\n\n    usageHistory(limit: 1, order_by: {createdAt: desc}) {\n      createdAt\n    }\n\n    teams_aggregate {\n      aggregate {\n        count\n      }\n    }\n\n    integrationInstallations_aggregate {\n      aggregate {\n        count\n      }\n    }\n\n    userSecret_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
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
        pub token: Vec<ProjectDetailsToken>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsToken {
        pub id: uuid,
        pub name: String,
        pub description: Option<String>,
        pub owner: ProjectDetailsTokenOwner,
        #[serde(rename = "usageHistory")]
        pub usage_history: Vec<ProjectDetailsTokenUsageHistory>,
        pub teams_aggregate: ProjectDetailsTokenTeamsAggregate,
        #[serde(rename = "integrationInstallations_aggregate")]
        pub integration_installations_aggregate:
            ProjectDetailsTokenIntegrationInstallationsAggregate,
        #[serde(rename = "userSecret_aggregate")]
        pub user_secret_aggregate: ProjectDetailsTokenUserSecretAggregate,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenOwner {
        pub id: uuid,
        pub name: String,
        pub email: String,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenUsageHistory {
        #[serde(rename = "createdAt")]
        pub created_at: timestamptz,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenTeamsAggregate {
        pub aggregate: Option<ProjectDetailsTokenTeamsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenTeamsAggregateAggregate {
        pub count: Int,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenIntegrationInstallationsAggregate {
        pub aggregate: Option<ProjectDetailsTokenIntegrationInstallationsAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenIntegrationInstallationsAggregateAggregate {
        pub count: Int,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenUserSecretAggregate {
        pub aggregate: Option<ProjectDetailsTokenUserSecretAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct ProjectDetailsTokenUserSecretAggregateAggregate {
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
