use crate::common::print_formatted_error::print_formatted_error;
use regex::Regex;

/// Validates a name for a secret field name.
/// The function performs the following checks:
/// - Ensures the name starts with letter, contains only alphanumeric characters, underscores, and hyphens.
/// - Verifies that the provided name is not already present in the list of `already_used_values`.
///
/// # Examples
///
/// ```rust
/// let used_names = vec!["existing_name".to_string(), "test_secret".to_string()];
/// let name = "new_secret";
/// let result = validate_secret_field_name(name, "", &used_names);
/// assert_eq!(result, Ok(()));
/// ```
///
/// # Arguments
///
/// * `name` - The name to validate.
/// * `default_value` - Current field name.
/// * `already_used_values` - A list of names that have already been used and should not be duplicated.
///
/// # Returns
///
/// Returns a `Result` with an empty tuple `()` if the name is valid. Otherwise, it returns an error message as a static string slice.
///
pub fn validate_secret_field_name(
    new_value: &str,
    default_value: &str,
    already_used_values: &Vec<String>,
) -> Result<(), &'static str> {
    let regex_start_result = Regex::new(r"^[a-zA-Z][\w ]*$");
    let regex_name_result = Regex::new(r"^[\w]*$");

    let regex_start = match regex_start_result {
        Ok(regex) => regex,

        Err(_e) => {
            print_formatted_error("Regular expression error checking the beginning of a name");
            std::process::exit(1);
        }
    };

    let regex_name = match regex_name_result {
        Ok(regex) => regex,

        Err(_e) => {
            print_formatted_error("Regular expression name check error");
            std::process::exit(1);
        }
    };

    if new_value.trim().len() < 2 {
        return Err("The field name must be at least 2 character long.");
    }

    if !regex_name.is_match(new_value) {
        return Err("For field name only only a-z, 0-9 and '_' are allowed.");
    }

    if !regex_start.is_match(new_value) {
        return Err("Field name should start with a letter.");
    }

    if already_used_values.contains(&new_value.to_string()) && new_value != default_value {
        return Err("The field name must be unique.");
    }

    Ok(())
}
