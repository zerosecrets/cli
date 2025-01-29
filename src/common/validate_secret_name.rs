use crate::common::print_formatted_error::print_formatted_error;

/// Validates a name for a secret or field.
/// The function performs the following checks:
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
    let new_value_trimmed_value = new_value.to_lowercase().trim().to_string();
    let default_value_trimmed_value = default_value.to_lowercase().trim().to_string();

    if new_value_trimmed_value.trim().chars().count() < 3 {
        print_formatted_error(
            "The secret name must be at least 3 characters long.",
        );

        std::process::exit(1);
    }

    if new_value_trimmed_value.trim().chars().count() > 32 {
        print_formatted_error(
            "The secret name must be less than 32 characters long.",
        );

        std::process::exit(1);
    }

    if already_used_values.contains(&new_value_trimmed_value.to_string())
        && new_value_trimmed_value != default_value_trimmed_value
    {
        return Err("The secret name must be unique.");
    }

    Ok(())
}
