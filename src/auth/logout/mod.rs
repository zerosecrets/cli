use crate::common::keyring::keyring;
use termimad::crossterm::style::Stylize;

pub fn logout() {
    keyring::delete("access_token");
    keyring::delete("user_id");
    println!("{} {}", "✓".green(), "You have successfully logged out.");
}
