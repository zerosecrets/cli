use dialoguer::{console::style, theme::ColorfulTheme};
use termimad::crossterm::style::Stylize;

pub fn theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_prefix: style("?".green().to_string()),
        ..ColorfulTheme::default()
    }
}
