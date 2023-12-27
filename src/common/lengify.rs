/// Shortens/extends a string to a fixed length, adding ellipsis if necessary.
///
/// # Arguments
/// `string` - Th string to shorten/expand.
///
/// # Returns
/// The string of fixed length
///
/// # Examples
/// ```
/// // "Short string     "
/// lengify("Short string");
///
/// // "Very very long..."
/// lengify("Very very long string");
/// ```
pub fn lengify(string: &str) -> String {
    let length = 32;

    if string.len() > length {
        format!("{}...", &string[..length - 3])
    } else {
        format!("{:width$}", string, width = length)
    }
}
