use regex::Regex;

pub enum Entity {
    Secret,
    Field,
}

/// Validates a name for a secret field name.
/// The function performs the following checks:
// FIXME update
/// - Ensures the name contains only alphanumeric characters, spaces, underscores, and hyphens.
/// - Verifies that the provided name is not already present in the list of `already_used_values`.
///
/// # Examples
///
/// ```rust
/// let used_names = vec!["existing_name".to_string(), "test_secret".to_string()];
/// let name = "new_secret";
/// let result = validate_name(name, &used_names, Entity::Secret);
/// assert_eq!(result, Ok(()));
/// ```
///
/// # Arguments
///
/// * `name` - The name to validate.
/// * `already_used_values` - A list of names that have already been used and should not be duplicated.
/// * `entity` - An enum specifying the type of entity (Entity::Secret or Entity::Field).
///
/// # Returns
///
/// Returns a `Result` with an empty tuple `()` if the name is valid. Otherwise, it returns an error message as a static string slice.
///
pub fn validate_secret_field_name(
    new_value: &str,
    default_value: &str,
    already_used_values: &Vec<String>,
    entity: Entity,
) -> Result<(), &'static str> {
    let regex_start = Regex::new(r"^[\w]*$").unwrap();
    let regex_name = Regex::new(r"^[a-zA-Z][\w]*$").unwrap();

    // TODO 2 simbols
    // Entity enum is used?

    // .test('name-symbols', 'For field name only a-z, and "_" are allowed', (value) =>
    // value ? /^[\w]*$/.test(value) : false,
    // )
    // .test('field-name-validation', 'Field name should start with a letter', (value) =>
    // value ? /^[a-zA-Z][\w]*$/.test(value) : false,

    // let blank_error_message = match entity {
    //     Entity::Secret => "The secret name must be at least 1 character long.",
    //     Entity::Field => "The field name must be at least 1 character long.",
    // };

    if new_value.trim().is_empty() {
        return Err("The field name must be at least 1 character long.");
    }

    if !regex_start.is_match(new_value) {
        return Err("Field name should start with a letter.");
    }

    if !regex_name.is_match(new_value) {
        return Err("For field name only only a-z, 0-9, ' ', '_', and '-' are allowed.");
    }

    // let uniqueness_error_message = match entity {
    //     Entity::Secret => "The secret name must be unique.",
    //     Entity::Field => "The field name must be unique.",
    // };

    if already_used_values.contains(&new_value.to_string()) && new_value != default_value {
        return Err("The field name must be unique.");
    }

    Ok(())
}
