use crate::common::print_formatted_error::print_formatted_error;
use regex::Regex;

/// Validates a name for a secret or field.
/// The function performs the following checks:
/// - Ensures the name contains only alphanumeric characters, spaces, underscores, and hyphens.
/// - Verifies that the provided name is not already present in the list of `already_used_values`.
///
/// # Examples
///
/// ```rust
/// let used_names = vec!["existing_name".to_string(), "test_secret".to_string()];
/// let name = "new_secret";
/// let result = validate_secret_name(name, "", &used_names);
/// assert_eq!(result, Ok(()));
/// ```
///
/// # Arguments
///
/// * `name` - The name to validate.
/// * `default_value` - Current secret name.
/// * `already_used_values` - A list of names that have already been used and should not be duplicated.
///
/// # Returns
///
/// Returns a `Result` with an empty tuple `()` if the name is valid. Otherwise, it returns an error message as a static string slice.
///
pub fn validate_secret_name(
    new_value: &str,
    default_value: &str,
    already_used_values: &Vec<String>,
) -> Result<(), &'static str> {
    let regex_result = Regex::new(r"^[\w -]+$");
    let new_value_trimmed_value = new_value.to_lowercase().trim().to_string();
    let default_value_trimmed_value = default_value.to_lowercase().trim().to_string();

    let regex = match regex_result {
        Ok(regex) => regex,

        Err(_e) => {
            print_formatted_error("Regular expression error checking the beginning of a name");
            std::process::exit(1);
        }
    };

    if new_value_trimmed_value.is_empty() {
        return Err("The secret name must be at least 1 character long.");
    }

    if !regex.is_match(&new_value_trimmed_value) {
        return Err("Only a-z, 0-9, ' ', '_', and '-' are allowed.");
    }

    if already_used_values.contains(&new_value_trimmed_value.to_string())
        && new_value_trimmed_value != default_value_trimmed_value
    {
        return Err("The secret name must be unique.");
    }

    Ok(())
}
