#![allow(clippy::all, warnings)]
pub struct UpdateTeamName;
pub mod update_team_name {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateTeamName";
    pub const QUERY : & str = "mutation UpdateTeamName($id: uuid!, $name: String!, $slug: String!) {\n  update_team_by_pk(pk_columns: { id: $id }, _set: { name: $name, slug: $slug }) {\n    id\n  }\n}\n" ;
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
        pub name: String,
        pub slug: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update_team_by_pk: Option<UpdateTeamNameUpdateTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct UpdateTeamNameUpdateTeamByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for UpdateTeamName {
    type Variables = update_team_name::Variables;
    type ResponseData = update_team_name::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_team_name::QUERY,
            operation_name: update_team_name::OPERATION_NAME,
        }
    }
}
