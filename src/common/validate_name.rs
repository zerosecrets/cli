use super::print_formatted_error::print_formatted_error;

/// Validates a name
///
/// # Examples
///
/// ```rust
/// let result = validate_name("new_name");
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
pub fn validate_name(name: &String) -> Result<(), &'static str> {
    let min_length = 3;
    let max_length = 32;
    let trimmed_name = name.trim();

    if trimmed_name.len() < min_length {
        print_formatted_error("Name must be at least 3 characters");
        std::process::exit(1);
    }

    if trimmed_name.len() > max_length {
        print_formatted_error("Name must be less than 32 characters long.");
        std::process::exit(1);
    }

    Ok(())
}
