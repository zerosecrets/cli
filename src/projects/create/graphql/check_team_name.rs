#![allow(clippy::all, warnings)]
pub struct CheckTeamName;
pub mod check_team_name {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "CheckTeamName";
    pub const QUERY : & str = "query CheckTeamName($userId: uuid, $name: String) {\n    team(where: {members: {member: {id: {_eq: $userId}}}, name: {_eq: $name}}) {\n        id\n    }\n}\n" ;
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
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<CheckTeamNameTeam>,
    }
    #[derive(Deserialize)]
    pub struct CheckTeamNameTeam {
        pub id: uuid::Uuid,
    }
}
impl graphql_client::GraphQLQuery for CheckTeamName {
    type Variables = check_team_name::Variables;
    type ResponseData = check_team_name::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: check_team_name::QUERY,
            operation_name: check_team_name::OPERATION_NAME,
        }
    }
}
