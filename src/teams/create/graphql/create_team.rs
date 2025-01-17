#![allow(clippy::all, warnings)]
pub struct CreateTeam;
pub mod create_team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateTeam";
    pub const QUERY : & str = "mutation CreateTeam($teamName: String!, $slug: String!) {\n  createTeam(name: $teamName, slug: $slug) {\n    id\n  }\n}\n" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "teamName")]
        pub team_name: String,
        pub slug: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createTeam")]
        pub create_team: CreateTeamCreateTeam,
    }
    #[derive(Deserialize)]
    pub struct CreateTeamCreateTeam {
        pub id: String,
    }
}
impl graphql_client::GraphQLQuery for CreateTeam {
    type Variables = create_team::Variables;
    type ResponseData = create_team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_team::QUERY,
            operation_name: create_team::OPERATION_NAME,
        }
    }
}
