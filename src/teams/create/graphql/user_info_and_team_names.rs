#![allow(clippy::all, warnings)]
pub struct UserInfoAndTeamNames;
pub mod user_info_and_team_names {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserInfoAndTeamNames";
    pub const QUERY : & str = "query UserInfoAndTeamNames($userId: uuid!) {\n    user_by_pk(id: $userId) {\n        id\n\n        userSubscription {\n            id\n            subscriptionPlan\n        }\n    }\n\n    team(where: {ownerUserId: {_eq: $userId}}) {\n        id\n        name\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[allow(non_camel_case_types)]
    pub enum subscriptionPlanEnum_enum {
        enterprise,
        free,
        professional,
        Other(String),
    }
    impl ::serde::Serialize for subscriptionPlanEnum_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                subscriptionPlanEnum_enum::enterprise => "enterprise",
                subscriptionPlanEnum_enum::free => "free",
                subscriptionPlanEnum_enum::professional => "professional",
                subscriptionPlanEnum_enum::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for subscriptionPlanEnum_enum {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "enterprise" => Ok(subscriptionPlanEnum_enum::enterprise),
                "free" => Ok(subscriptionPlanEnum_enum::free),
                "professional" => Ok(subscriptionPlanEnum_enum::professional),
                _ => Ok(subscriptionPlanEnum_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "userId")]
        pub user_id: uuid::Uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub user_by_pk: Option<UserInfoAndTeamNamesUserByPk>,
        pub team: Vec<UserInfoAndTeamNamesTeam>,
    }
    #[derive(Deserialize)]
    pub struct UserInfoAndTeamNamesUserByPk {
        pub id: uuid::Uuid,
        #[serde(rename = "userSubscription")]
        pub user_subscription: Option<UserInfoAndTeamNamesUserByPkUserSubscription>,
    }
    #[derive(Deserialize)]
    pub struct UserInfoAndTeamNamesUserByPkUserSubscription {
        pub id: uuid::Uuid,
        #[serde(rename = "subscriptionPlan")]
        pub subscription_plan: subscriptionPlanEnum_enum,
    }
    #[derive(Deserialize)]
    pub struct UserInfoAndTeamNamesTeam {
        pub id: uuid::Uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for UserInfoAndTeamNames {
    type Variables = user_info_and_team_names::Variables;
    type ResponseData = user_info_and_team_names::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_info_and_team_names::QUERY,
            operation_name: user_info_and_team_names::OPERATION_NAME,
        }
    }
}
