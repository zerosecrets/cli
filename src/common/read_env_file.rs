use std::{self, collections::BTreeMap, fs::OpenOptions, io::BufRead, io::BufReader};

/// Reads an environment file from the specified `file_path` and parses its contents into a `BTreeMap`.
///
/// # Arguments
///
/// * `file_path` - A string slice that represents the path to the environment file.
///
/// # Returns
///
/// Returns a `BTreeMap<String, String>` containing key-value pairs parsed from the environment file.
///
/// # Errors
///
/// This function may return an error if there are issues opening or reading the file.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
///
/// let env_map = read_env_file("/path/to/file.env");
/// println!("{:?}", env_map);
/// ```
pub fn read_env_file(file_path: &str) -> BTreeMap<String, String> {
    let mut env_vars = BTreeMap::new();

    if let Ok(file) = OpenOptions::new().read(true).open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.splitn(2, '=').collect();

                if parts.len() == 2 {
                    env_vars.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    }

    env_vars
}
