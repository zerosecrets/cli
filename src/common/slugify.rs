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
    // 1) convert everything to lowercase
    let mut slug = text.to_lowercase();

    slug = slug.trim().to_string();

    // 3) remove everything that isn't in [a-z0-9, space, -, _]
    //    (filter by characters)
    let filtered: String = slug
        .chars()
        .filter(|c| {
            c.is_ascii_alphanumeric() // [a-z, 0-9]
                || *c == ' '
                || *c == '-'
                || *c == '_'
        })
        .collect();

    // 4) split by any sequence of space, '_', or '-',
    //    then join with "-"
    //    this automatically removes consecutive characters
    //    and leading/trailing hyphens
    filtered
        .split(|c: char| c == ' ' || c == '_' || c == '-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!"), "hello-world");

        assert_eq!(
            slugify("  multiple   spaces   here  "),
            "multiple-spaces-here"
        );

        assert_eq!(slugify("__already-slugified__"), "already-slugified");
        assert_eq!(slugify("UPPER_case---stuff###"), "upper-case-stuff");
        assert_eq!(slugify("hello-_-world"), "hello-world");
        assert_eq!(slugify("*****"), "");
    }
}
