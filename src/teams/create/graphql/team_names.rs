#![allow(clippy::all, warnings)]
pub struct TeamNames;
pub mod team_names {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamNames";
    pub const QUERY : & str = "query TeamNames($userId: uuid!) {\n  team(where: { ownerUserId: { _eq: $userId } }) {\n    id\n    name\n  }\n}\n" ;
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
        #[serde(rename = "userId")]
        pub user_id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<TeamNamesTeam>,
    }
    #[derive(Deserialize)]
    pub struct TeamNamesTeam {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for TeamNames {
    type Variables = team_names::Variables;
    type ResponseData = team_names::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_names::QUERY,
            operation_name: team_names::OPERATION_NAME,
        }
    }
}
