use termimad::crossterm::style::Stylize;

/// Prints a formatted error message to the standard error (stderr) stream.
///
/// This function takes an error message as input, formats it by prepending "Error: ",
/// and then prints it to the standard error stream (usually the terminal).
///
/// # Arguments
///
/// * `error_message` - The error message to be printed.
///
/// # Examples
///
/// ```
/// let error_message = "Failed to open the file.";
/// print_formatted_error(error_message);
/// ```
///
pub fn print_formatted_error(error_message: &str) {
    let formatted_error_message = format!("Error: {}", error_message);
    eprintln!("{}", formatted_error_message.red());
}
