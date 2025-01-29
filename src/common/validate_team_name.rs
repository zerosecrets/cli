/// Validates a name for a team
///
/// # Examples
///
/// ```rust
/// let used_names = vec!["existing_name".to_string(), "test_secret".to_string()];
/// let result = validate_secret_name("new_team", used_names);
/// assert_eq!(result, Ok(()));
/// ```
///
/// # Arguments
///
/// * `name` - The name to validate.
/// * `existing_names` - A list of names that have already been used and should not be duplicated.
///
///
/// # Returns
///
/// Returns a `Result` with an empty tuple `()` if the name is valid. Otherwise, it returns an error message as a static string slice.
///
pub fn validate_team_name(name: &String, existing_names: &Vec<String>) -> Result<(), String> {
  let min_length = 3;
  let max_length = 32;

  if name.trim().len() < min_length {
      return Err(format!(
          "Team name must be at least {} characters",
          min_length
      ));
  }

  if name.trim().len() > min_length {
      return Err(format!(
          "Team name must be less than {} characters long.",
          max_length
      ));
  }

  if existing_names.contains(&name.trim().to_string()) {
      return Err("Team name is already in use".to_string());
  }

  Ok(())
}
