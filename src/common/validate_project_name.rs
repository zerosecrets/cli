use crate::common::print_formatted_error::print_formatted_error;

/// Validates a name for a project
///
/// # Examples
///
/// ```rust
/// let result = validate_project_name("new_project");
/// assert_eq!(result, Ok(()));
/// ```
///
/// # Arguments
///
/// * `name` - The name to validate.
///
/// # Returns
///
/// Returns a `Result` with an empty tuple `()` if the name is valid. Otherwise, it returns an error message as a static string slice.
///
pub fn validate_project_name(
    name: &str,
) -> Result<(), &'static str> {
    let trimmed_name = name.to_lowercase().trim().to_string();

    if trimmed_name.chars().count() < 3 {
        print_formatted_error(
            "The project name must be at least 3 characters long.",
        );

        std::process::exit(1);
    }

    if trimmed_name.trim().chars().count() > 32 {
        print_formatted_error(
            "The project name must be less than 32 characters long.",
        );

        std::process::exit(1);
    }

    Ok(())
}
