use crate::common::{
    authorization_headers::authorization_headers,
    execute_graphql_request::execute_graphql_request,
    graphql::me::{me, Me},
};

use graphql_client::GraphQLQuery;
use reqwest::Client;
use uuid::Uuid;

/// Fetches the user ID associated with the provided access token and outputs it.
/// This function sends a GraphQL query to retrieve the user's ID using the provided access token.
///
/// # Arguments
///
/// * `token` - A valid access token for authentication.
///
/// # Returns
///
/// The user's ID as a UUID (Universally Unique Identifier).
///
/// # Examples
///
/// ```
/// let access_token = "your_access_token";
/// let user_id = fetch_user_id(access_token);
/// println!("User ID: {}", user_id);
/// ```
///
/// # Panics
///
/// This function will panic and terminate the program if any errors occur during the process,
/// including if the `GRAPHQL_URL` environment variable is not set, if the authorization header is missing,
/// or if there are errors in the GraphQL response.
///
pub fn fetch_user_id(access_token: &str) -> Uuid {
    let user_id = execute_graphql_request::<me::Variables, me::ResponseData>(
        authorization_headers(&access_token),
        Me::build_query,
        &Client::new(),
        "Failed to retrieve a user ID",
        me::Variables,
    )
    .me
    .id;

    return user_id;
}
