use crate::common::print_formatted_error::print_formatted_error;
use reqwest::header;

/// Constructs and returns authorization headers for making authenticated HTTP requests
///
/// # Arguments
///
/// * `access_token` - A string representing the access token for authentication.
///
/// # Returns
///
/// A `HeaderMap` containing the "Authorization" header with the provided access token.
///
/// # Panics
///
/// This function will panic and exit the process with an error message if it fails to
/// create the "Authorization" header from the provided access token.
///
/// # Example
///
/// ```
/// use common::authorization_headers;
///
/// let access_token = "your_access_token";
/// let headers = authorization_headers(access_token);
/// ```
///
/// The function returns a `HeaderMap` that can be used for making authenticated HTTP requests.
///
/// Note: Ensure that the provided access token is valid and properly formatted.
pub fn authorization_headers(access_token: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    let header_value = match header::HeaderValue::from_str(&format!("Bearer {}", access_token)) {
        Ok(value) => value,
        Err(_) => {
            print_formatted_error("Service error. Please try to log in again.");
            std::process::exit(1);
        }
    };

    headers.insert("Authorization", header_value);
    headers
}
