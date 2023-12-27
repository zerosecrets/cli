#![allow(clippy::all, warnings)]
pub struct UserPersonalProjectTeamId;
pub mod user_personal_project_team_id {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "UserPersonalProjectTeamId";
    pub const QUERY : & str = "query UserPersonalProjectTeamId($userId: uuid!) {\n   team(where: {_and: [\n    {name: {_eq: \"Personal projects\"}}\n    {ownerUserId: {_eq: $userId}}\n  ]}) {\n    id\n  }\n}\n" ;
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
        #[serde(rename = "userId")]
        pub user_id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<UserPersonalProjectTeamIdTeam>,
    }
    #[derive(Deserialize)]
    pub struct UserPersonalProjectTeamIdTeam {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for UserPersonalProjectTeamId {
    type Variables = user_personal_project_team_id::Variables;
    type ResponseData = user_personal_project_team_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_personal_project_team_id::QUERY,
            operation_name: user_personal_project_team_id::OPERATION_NAME,
        }
    }
}
