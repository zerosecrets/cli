/// # keyring
///
/// The `keyring` module provides utility functions for working with the system's keyring or password
/// manager. It allows you to securely store, retrieve, and delete key-value pairs associated with a
/// specific service name.
///
/// ## Example
///
/// ```rust
/// use zero_cli::keyring;
///
/// // Set a key-value pair in the keyring
/// keyring::set("my_key", "my_password");
///
/// // Retrieve the value associated with the key
/// let password = keyring::get("my_key");
/// println!("Password: {}", password);
///
/// // Delete the key-value pair from the keyring
/// keyring::delete("my_key");
/// ```
pub mod keyring {
    use crate::common::print_formatted_error::print_formatted_error;
    use keyring::Entry;

    /// The name of the service for which you want to store key-value pairs in the keyring.
    const SERVICE_NAME: &str = "zero-cli";

    pub fn set(key: &str, value: &str) -> () {
        let entry_result = Entry::new(SERVICE_NAME, key);
        let set_error_message = "Authorization error. Failed to set the token to the device.";

        match entry_result {
            Ok(entry) => match entry.set_password(value) {
                Ok(_) => (),

                Err(_) => {
                    print_formatted_error(set_error_message);
                    std::process::exit(1);
                }
            },

            Err(_) => {
                print_formatted_error(set_error_message);
                std::process::exit(1);
            }
        }
    }

    pub fn get(key: &str) -> String {
        let entry = Entry::new(SERVICE_NAME, key);
        let get_error_message = "You are not logged in, please enter the command 'zero-cli auth login', and log in using your tryzero.com account.";

        match entry {
            Ok(entry) => match entry.get_password() {
                Ok(token) => token,

                Err(_) => {
                    print_formatted_error(get_error_message);
                    std::process::exit(1);
                }
            },

            Err(_) => {
                print_formatted_error(get_error_message);
                std::process::exit(1);
            }
        }
    }

    pub fn delete(key: &str) -> () {
        let entry_result = Entry::new(SERVICE_NAME, key);
        let delete_error_message = "You are not logged in.";

        match entry_result {
            Ok(entry) => match entry.delete_password() {
                Ok(_) => (),

                Err(_) => {
                    print_formatted_error(delete_error_message);
                    std::process::exit(1);
                }
            },

            Err(_) => {
                print_formatted_error(delete_error_message);
                std::process::exit(1);
            }
        }
    }
}
