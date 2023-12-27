use crate::common::print_formatted_error::print_formatted_error;
use tokio::runtime;

/// Creates and returns a new Tokio runtime for blocking tasks.
///
/// This function initializes and returns a new Tokio runtime, which can be used for running asynchronous
/// code in a blocking context. If the runtime creation is successful, it returns the runtime.
/// If there is an error during runtime creation, an error message is printed to the standard error (stderr)
/// stream, and the program exits with a status code of 1.
///
/// # Returns
///
/// A new Tokio runtime.
///
/// # Panics
///
/// This function will panic and terminate the program if it fails to create a new Tokio runtime.
///
/// # Examples
///
/// ```
/// let runtime = runtime_block_on();
/// // Use the `runtime` for blocking tasks.
/// ```
///
pub fn tokio_runtime() -> runtime::Runtime {
    match runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(_) => {
            print_formatted_error("Service error. Please try again.");
            std::process::exit(1);
        }
    }
}
