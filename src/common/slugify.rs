use dialoguer::{theme::ColorfulTheme, Input};
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
use regex::Regex;

use super::print_formatted_error::print_formatted_error;

///
pub fn slugify(text: &str) -> String {
    // convert everything to lowercase
    let mut slug = text.to_lowercase();

    slug = slug.trim().to_string();

    // remove everything that isn't in [a-z0-9, space, -, _]
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

    // split by any sequence of space, '_', or '-',
    //    then join with "-"
    //    this automatically removes consecutive characters
    //    and leading/trailing hyphens
    filtered
        .split(|c: char| c == ' ' || c == '_' || c == '-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn validate_slug(slug: &String) -> Result<(), String> {
    let min_length = 3;
    let regex = Regex::new(r"^[a-z\d]+(?:-[a-z\d]+)*$").unwrap();

    if slug.trim().len() < min_length {
        return Err(format!("Slug must be at least {} characters", min_length));
    }

    if !regex.is_match(slug.trim()) {
        return Err(
            "Only a-z, 0-9, and single hyphens (not at the start or end) are allowed.".to_string(),
        );
    }

    Ok(())
}

pub fn slugify_prompt(text: &str, prompt_message: &str) -> String {
    return match Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_message)
        .default(slugify(text))
        .validate_with(validate_slug)
        .interact()
    {
        Ok(slug) => slug.trim().to_owned(),

        Err(_) => {
            print_formatted_error("Failed to read slug input.");
            std::process::exit(1);
        }
    };
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
