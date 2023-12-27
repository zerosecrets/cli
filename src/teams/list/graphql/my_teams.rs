#![allow(clippy::all, warnings)]
pub struct MyTeams;
pub mod my_teams {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "MyTeams";
    pub const QUERY : & str = "query MyTeams($userId: uuid!) {\n  team(\n    where: {\n      _and: [\n        {members: {member: {id: {_eq: $userId}}}}\n        {ownerUserId: {_eq: $userId}}\n        {owner: {userSubscription: {subscriptionPlan: {_neq: free}}}}\n      ]\n    }\n    order_by: {lastUpdatedAt: desc}\n    limit: 1000\n  ) {\n    id\n    name\n\n    owner {\n      id\n      name\n    }\n\n    members_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
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
        pub team: Vec<MyTeamsTeam>,
    }
    #[derive(Deserialize)]
    pub struct MyTeamsTeam {
        pub id: uuid,
        pub name: String,
        pub owner: Option<MyTeamsTeamOwner>,
        pub members_aggregate: MyTeamsTeamMembersAggregate,
    }
    #[derive(Deserialize)]
    pub struct MyTeamsTeamOwner {
        pub id: uuid,
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct MyTeamsTeamMembersAggregate {
        pub aggregate: Option<MyTeamsTeamMembersAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct MyTeamsTeamMembersAggregateAggregate {
        pub count: Int,
    }
}
impl graphql_client::GraphQLQuery for MyTeams {
    type Variables = my_teams::Variables;
    type ResponseData = my_teams::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: my_teams::QUERY,
            operation_name: my_teams::OPERATION_NAME,
        }
    }
}
