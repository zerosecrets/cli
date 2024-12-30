#![allow(clippy::all, warnings)]
pub struct RemoveUserFromTeam;
pub mod remove_user_from_team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RemoveUserFromTeam";
    pub const QUERY : & str = "mutation RemoveUserFromTeam(\n  $teamId: String!\n  $userId: String!\n  $ownerTeamUserId: String!\n) {\n  removeUserFromTeam(\n    teamId: $teamId\n    userId: $userId\n    ownerTeamUserId: $ownerTeamUserId\n  ) {\n    teamId\n  }\n}\n" ;
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
        #[serde(rename = "teamId")]
        pub team_id: String,
        #[serde(rename = "userId")]
        pub user_id: String,
        #[serde(rename = "ownerTeamUserId")]
        pub owner_team_user_id: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "removeUserFromTeam")]
        pub remove_user_from_team: RemoveUserFromTeamRemoveUserFromTeam,
    }
    #[derive(Deserialize)]
    pub struct RemoveUserFromTeamRemoveUserFromTeam {
        #[serde(rename = "teamId")]
        pub team_id: String,
    }
}
impl graphql_client::GraphQLQuery for RemoveUserFromTeam {
    type Variables = remove_user_from_team::Variables;
    type ResponseData = remove_user_from_team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: remove_user_from_team::QUERY,
            operation_name: remove_user_from_team::OPERATION_NAME,
        }
    }
}
