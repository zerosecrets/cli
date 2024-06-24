#![allow(clippy::all, warnings)]
pub struct DeleteTeam;
pub mod delete_team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteTeam";
    pub const QUERY: &str =
        "mutation DeleteTeam($id: uuid!) {\n  delete_team_by_pk(id: $id) {\n    id\n  }\n}\n";
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
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub delete_team_by_pk: Option<DeleteTeamDeleteTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct DeleteTeamDeleteTeamByPk {
        pub id: uuid,
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
