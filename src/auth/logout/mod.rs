use crate::common::keyring::keyring;
use termimad::crossterm::style::Stylize;

pub fn logout() {
    keyring::delete("access_token");
    println!("{} {}", "✓".green(), "You have successfully logged out.");
}
