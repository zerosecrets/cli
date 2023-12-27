use crate::common::print_formatted_error::print_formatted_error;
use std::{collections::BTreeMap, fs::OpenOptions, io::Write};

/// Writes the contents of a `BTreeMap` with environment variables to the specified file path.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path to the file where the environment variables will be written.
/// * `env_vars` - A reference to a `BTreeMap<String, String>` containing environment variable key-value pairs.
///
/// # Errors
///
/// This function may return an error if there are issues opening or writing to the file.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
///
/// let mut env_map = BTreeMap::new();
/// env_map.insert("KEY".to_string(), "VALUE".to_string());
/// write_env_file("/path/to/file.env", &env_map);
/// ```
pub fn write_env_file(file_path: &str, env_vars: &BTreeMap<String, String>) {
    let sorted_env_vars: BTreeMap<String, String> = env_vars
        .iter()
        .map(|(key, value)| (key.to_uppercase().replace('-', "_"), value.clone()))
        .collect();

    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
    {
        Ok(mut file) => {
            for (key, value) in sorted_env_vars {
                if let Err(err) = writeln!(&mut file, "{}={}", key, value) {
                    print_formatted_error(&format!("Failed to write to file: {}", err));
                    std::process::exit(1);
                }
            }
        }

        Err(_) => {
            print_formatted_error("Failed to open file for writing.");
            std::process::exit(1);
        }
    }
}
