mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, execute_graphql_request::execute_graphql_request,
};

use crate::common::print_formatted_error::print_formatted_error;
use graphql::team_info::{team_info, TeamInfo};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use uuid::Uuid;

pub fn team_info(access_token: &str, team_id: Uuid) -> team_info::TeamInfoTeamByPk {
    let error_message = "Failed to retrieve a team info.";

    match execute_graphql_request::<team_info::Variables, team_info::ResponseData>(
        authorization_headers(&access_token),
        TeamInfo::build_query,
        &Client::new(),
        error_message,
        team_info::Variables { team_id },
    )
    .team_by_pk
    {
        Some(data) => data,

        None => {
            print_formatted_error(error_message);
            std::process::exit(1);
        }
    }
}
