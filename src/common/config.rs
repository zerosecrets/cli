/// Struct to hold the configuration of the CLI
pub struct Config {
    pub graphql_url: String,
    pub items_per_page: usize,
    pub min_id_length: usize,
    pub webapp_cli_auth_url: String,
    pub webapp_url: String,
    pub date_format: String,
}

/// Default implementation for the Config struct
impl Config {
    pub fn new() -> Self {
        Self {
            date_format: String::from("%B %d %Y, %H:%M:%S"),
            graphql_url: String::from("https://api.tryzero.com/v1/graphql"),
            items_per_page: 10,
            min_id_length: 4,
            webapp_cli_auth_url: String::from("https://tryzero.com/cli-success-login"),
            webapp_url: String::from("https://tryzero.com"),
        }
    }
}
