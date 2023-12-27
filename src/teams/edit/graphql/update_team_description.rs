#![allow(clippy::all, warnings)]
pub struct UpdateTeamDescription;
pub mod update_team_description {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UpdateTeamDescription";
    pub const QUERY : & str = "mutation UpdateTeamDescription($id: uuid!, $description: String!) {\n  update_team_by_pk(pk_columns: {id: $id}, _set: {description: $description}) {\n    id\n  }\n}\n" ;
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
        pub description: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub update_team_by_pk: Option<UpdateTeamDescriptionUpdateTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct UpdateTeamDescriptionUpdateTeamByPk {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for UpdateTeamDescription {
    type Variables = update_team_description::Variables;
    type ResponseData = update_team_description::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_team_description::QUERY,
            operation_name: update_team_description::OPERATION_NAME,
        }
    }
}
