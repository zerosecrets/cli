/// Pads the input `key` to fit a specified column width.
///
/// # Arguments
///
/// * `key` - A string slice that holds the value to be formatted.
/// * `width` - The column width to which `key` should be padded.
///
/// # Returns
///
/// Returns a `String` that represents the padded value of `key`.
///
/// # Examples
///
/// ```
/// let formatted = pad_to_column_width("example", 10);
/// assert_eq!(formatted, "example   ");
/// ```
///
pub fn pad_to_column_width(key: String, width: usize) -> String {
    format!("{}", format!("{:width$}", key, width = width))
}
