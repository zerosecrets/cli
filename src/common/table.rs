use crate::common::{
    config::Config, list::list::List, print_formatted_error::print_formatted_error,
};
use dialoguer::{console::style, theme::ColorfulTheme};

/// Displays a paginated table of items with user interaction.
///
/// `table` renders a paginated table of items on the terminal with the ability for the user
/// to navigate through pages using arrow keys. It utilizes the `dialoguer` crate to handle user input.
///
/// # Arguments
///
/// * `items` - A vector of strings representing items to display in the table.
/// * `description` - A description message displayed to the user.
/// * `error_message` - An error message to display if there's an issue during interaction.
/// * `table_headers` - A string representing the table headers.
///
/// # Examples
///
/// ```
/// use common::table::table;
///
/// let items = vec![
///     "Item 1".to_string(),
///     "Item 2".to_string(),
///     // Add more items...
/// ];
///
/// table(items, "Choose an item:".to_string(), "Error occurred during selection.", "ID     NAME    OWNER");
/// ```
///
/// # Panics
///
/// Panics if parsing the number of items per page fails, printing a formatted error message.
///
pub fn table(items: Vec<String>, description: String, empty_message: &str, table_headers: String) {
    let theme = ColorfulTheme {
        prompt_prefix: style(" ".to_string()),
        prompt_suffix: style("".to_string()),
        ..ColorfulTheme::default()
    };

    let items_per_page = Config::new().items_per_page;

    if items.len() == 0 {
        println!("{}", empty_message);
        std::process::exit(0);
    }

    match List::with_theme(&theme)
        .with_prompt(format!("{}", table_headers))
        .with_description(description)
        .max_length(items_per_page)
        .items(&items)
        .clear(true)
        .interact_opt()
    {
        Ok(_) => {
            std::process::exit(0);
        }

        Err(_) => {
            print_formatted_error("Service error. Please try again.");
            std::process::exit(1);
        }
    };
}
