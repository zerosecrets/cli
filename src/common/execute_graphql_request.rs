use crate::common::{
    config::Config, print_formatted_error::print_formatted_error, tokio_runtime::tokio_runtime,
};
use graphql_client::Response;
use reqwest::{header::HeaderMap, Client};
use spinners::{Spinner, Spinners};

/// Executes a GraphQL request and returns the response data.
///
/// This function takes care of sending a GraphQL request to the specified GraphQL server,
/// handling errors, and parsing the response data into the specified type `D`.
///
/// # Arguments
///
/// * `authorization_headers` - The authorization headers to include in the request.
/// * `build_query_fn` - A function that builds the GraphQL query body based on the provided variables.
/// * `client` - The Reqwest HTTP client used to make the request.
/// * `error_message` - The error message to display and exit with if an error occurs.
/// * `variables` - The variables to include in the GraphQL request.
///
/// # Returns
///
/// The response data of type `D` from the GraphQL server.
///
/// # Panics
///
/// This function will panic and terminate the program if there are errors during the request or
/// if the response does not contain valid data.
///
/// # Example
///
/// ```rust
/// use reqwest::Client;
/// use graphql_client::QueryBody;
///
/// // Define a custom GraphQL query type.
/// struct MyQuery;
/// struct MyVariables;
/// struct MyResponseData;
///
/// // Implement the `GraphQLQuery` trait for the custom query type.
/// impl graphql_client::GraphQLQuery for MyQuery {
///     type Variables = MyVariables;
///     type ResponseData = MyResponseData;
///     fn build_query(variables: Self::Variables) -> QueryBody<Self::Variables> {
///         // Build and return the GraphQL query body here.
///         // This function should be specific to your GraphQL schema.
///         unimplemented!();
///     }
/// }
///
/// // Define authorization headers, client, and variables.
/// let authorization_headers = HeaderMap::new();
/// let client = Client::new();
/// let variables = MyVariables;
///
/// // Execute the GraphQL request.
/// let response_data: MyResponseData = execute_graphql_request(
///     authorization_headers,
///     MyQuery::build_query,
///     &client,
///     "Failed to fetch data from GraphQL server.",
///     variables,
/// );
/// ```
pub fn execute_graphql_request<V, D>(
    authorization_headers: HeaderMap,
    build_query_fn: fn(V) -> graphql_client::QueryBody<V>,
    client: &Client,
    error_message: &str,
    variables: V,
) -> D
where
    V: serde::Serialize,
    D: for<'a> serde::Deserialize<'a>,
{
    let mut sp = Spinner::new(Spinners::Dots, String::new());

    let response = tokio_runtime().block_on(async {
        match client
            .post(Config::new().graphql_url)
            .json(&build_query_fn(variables))
            .headers(authorization_headers)
            .send()
            .await
        {
            Ok(response) => {
                let response_data: Response<D> = match response.json().await {
                    Ok(response_data) => response_data,

                    Err(_) => {
                        print_formatted_error(error_message);
                        std::process::exit(1);
                    }
                };

                if let Some(_) = response_data.errors {
                    print_formatted_error(error_message);
                    std::process::exit(1);
                }

                if let Some(data) = response_data.data {
                    data
                } else {
                    print_formatted_error(error_message);
                    std::process::exit(1);
                }
            }

            Err(_) => {
                print_formatted_error(error_message);
                std::process::exit(1);
            }
        }
    });

    sp.stop_and_persist("", "".to_string());
    response
}
