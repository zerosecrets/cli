#![allow(clippy::all, warnings)]
pub struct DeleteTeam;
pub mod delete_team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteTeam";
    pub const QUERY: &str =
        "mutation DeleteTeam($id: ID!) {\n  removeTeam(teamId: $id) {\n    id\n  }\n}\n";
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
        pub id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "removeTeam")]
        pub remove_team: DeleteTeamRemoveTeam,
    }
    #[derive(Deserialize)]
    pub struct DeleteTeamRemoveTeam {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for DeleteTeam {
    type Variables = delete_team::Variables;
    type ResponseData = delete_team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_team::QUERY,
            operation_name: delete_team::OPERATION_NAME,
        }
    }
}
