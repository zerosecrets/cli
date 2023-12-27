#![allow(clippy::all, warnings)]
pub struct SendInvite;
pub mod send_invite {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "SendInvite";
    pub const QUERY : & str = "mutation SendInvite($teamId: String!, $email: String!) {\n    sendInviteUserTeam(object: {teamId: $teamId, email: $email}) {\n        success\n    }\n}\n" ;
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
        pub email: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "sendInviteUserTeam")]
        pub send_invite_user_team: SendInviteSendInviteUserTeam,
    }
    #[derive(Deserialize)]
    pub struct SendInviteSendInviteUserTeam {
        pub success: Boolean,
    }
}
impl graphql_client::GraphQLQuery for SendInvite {
    type Variables = send_invite::Variables;
    type ResponseData = send_invite::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: send_invite::QUERY,
            operation_name: send_invite::OPERATION_NAME,
        }
    }
}
