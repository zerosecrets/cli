#![allow(clippy::all, warnings)]
pub struct TeamIdBySlug;
pub mod team_id_by_slug {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamIdBySlug";
    pub const QUERY : & str = "query TeamIdBySlug($slug: String!) {\n  team(where: { slug: { _eq: $slug } }) {\n    id\n  }\n}\n" ;
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
        pub slug: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<TeamIdBySlugTeam>,
    }
    #[derive(Deserialize)]
    pub struct TeamIdBySlugTeam {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for TeamIdBySlug {
    type Variables = team_id_by_slug::Variables;
    type ResponseData = team_id_by_slug::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_id_by_slug::QUERY,
            operation_name: team_id_by_slug::OPERATION_NAME,
        }
    }
}
