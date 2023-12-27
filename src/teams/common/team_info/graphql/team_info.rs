#![allow(clippy::all, warnings)]
pub struct TeamInfo;
pub mod team_info {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "TeamInfo";
    pub const QUERY : & str = "query TeamInfo($teamId: uuid!) {\n  team_by_pk(id: $teamId) {\n    id\n    name\n    description\n    ownerUserId\n\n    members {\n      id\n\n      member {\n        id\n      }\n    }\n  }\n}\n" ;
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
        #[serde(rename = "teamId")]
        pub team_id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team_by_pk: Option<TeamInfoTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct TeamInfoTeamByPk {
        pub id: uuid::Uuid,
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "ownerUserId")]
        pub owner_user_id: uuid::Uuid,
        pub members: Vec<TeamInfoTeamByPkMembers>,
    }
    #[derive(Deserialize)]
    pub struct TeamInfoTeamByPkMembers {
        pub id: uuid::Uuid,
        pub member: TeamInfoTeamByPkMembersMember,
    }
    #[derive(Deserialize)]
    pub struct TeamInfoTeamByPkMembersMember {
        pub id: uuid::Uuid,
    }
}
impl graphql_client::GraphQLQuery for TeamInfo {
    type Variables = team_info::Variables;
    type ResponseData = team_info::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_info::QUERY,
            operation_name: team_info::OPERATION_NAME,
        }
    }
}
