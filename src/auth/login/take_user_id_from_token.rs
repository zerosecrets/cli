use crate::common::print_formatted_error::print_formatted_error;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct HasuraClaims {
    #[serde(rename = "x-hasura-allowed-roles")]
    allowed_roles: Vec<String>,
    #[serde(rename = "x-hasura-default-role")]
    default_role: String,
    #[serde(rename = "x-hasura-user-id")]
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    role: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "https://hasura.io/jwt/claims")]
    hasura_claims: HasuraClaims,
    iat: u64,
    exp: u64,
    iss: String,
}

fn pad_base64(input: &str) -> String {
    let mut output = input.to_string();
    while output.len() % 4 != 0 {
        output.push('=');
    }
    output
}

pub fn take_user_id_from_token(token: &str) -> String {
    let token_parts: Vec<&str> = token.split('.').collect();

    if token_parts.len() != 3 {
        print_formatted_error(&format!("Invalid token format"));
        std::process::exit(1);
    }

    let payload_encoded = pad_base64(token_parts[1]);

    return match general_purpose::URL_SAFE.decode(&payload_encoded) {
        Ok(decoded_bytes) => match str::from_utf8(&decoded_bytes) {
            Ok(payload_json) => match serde_json::from_str::<Claims>(payload_json) {
                Ok(claims) => claims.hasura_claims.user_id,

                Err(err) => {
                    print_formatted_error(&format!("Error parsing claims: {:?}", err));
                    std::process::exit(1);
                }
            },

            Err(err) => {
                print_formatted_error(&format!("Error decoding payload: {:?}", err));
                std::process::exit(1);
            }
        },

        Err(err) => {
            print_formatted_error(&format!("Error decoding base64: {:?}", err));
            std::process::exit(1);
        }
    };
}
