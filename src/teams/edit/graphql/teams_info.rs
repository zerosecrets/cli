#![allow(clippy::all, warnings)]
pub struct TeamsInfo;
pub mod teams_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamsInfo";
    pub const QUERY: &str =
        "query TeamsInfo {\n  team {\n    id\n    name\n    description\n    slug\n  }\n}\n";
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
    type uuid = ::uuid::Uuid;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<TeamsInfoTeam>,
    }
    #[derive(Deserialize)]
    pub struct TeamsInfoTeam {
        pub id: uuid,
        pub name: String,
        pub description: String,
        pub slug: String,
    }
}
impl graphql_client::GraphQLQuery for TeamsInfo {
    type Variables = teams_info::Variables;
    type ResponseData = teams_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: teams_info::QUERY,
            operation_name: teams_info::OPERATION_NAME,
        }
    }
}
