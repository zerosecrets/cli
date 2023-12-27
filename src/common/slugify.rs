/// This function takes a text string as input and performs the following transformations:
/// - Removes special characters, keeping only alphanumeric characters, spaces, and hyphens.
/// - Trims leading and trailing spaces from the resulting string.
/// - Replaces spaces with hyphens.
/// - Converts the string to lowercase.
///
/// # Examples
///
/// ```
/// let input = "This is an Example!";
/// let result = slugify(input);
/// assert_eq!(result, "this-is-an-example");
/// ```
///
/// # Arguments
///
/// * `text` - A string slice that you want to slugify.
///
/// # Returns
///
/// A new formatted String.
pub fn slugify(text: &str) -> String {
    let replaced = text
        .chars()
        .filter(|char| char.is_alphanumeric() || *char == ' ' || *char == '-')
        .collect::<String>();

    let trimmed = replaced.trim();

    let slug = trimmed
        .chars()
        .map(|c| if c == ' ' { '-' } else { c })
        .collect::<String>()
        .to_lowercase();

    slug
}
