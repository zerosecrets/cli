use std::collections::HashMap;
use std::sync::RwLock;

pub mod keyring {
    use super::*;
    use std::sync::Arc;

    /// The name of the service for which you want to store key-value pairs in the keyring.
    const SERVICE_NAME: &str = "zero-cli";

    // Структура для хранения состояния
    struct KeyringState {
        storage: RwLock<HashMap<String, String>>,
    }

    impl KeyringState {
        fn new() -> Self {
            let mut initial_storage = HashMap::new();
            initial_storage.insert("access_token".to_string(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0eXBlIjoiYWNjZXNzVG9rZW4iLCJlbWFpbCI6InBsYW5AZXhhbXBsZS5jb20iLCJyb2xlIjoidXNlciIsInVzZXJJZCI6ImQ4NDM4ZWQ1LWZkNzctNDYzYy05NDJlLTY2OWE3NzRkZDc3NyIsImh0dHBzOi8vaGFzdXJhLmlvL2p3dC9jbGFpbXMiOnsieC1oYXN1cmEtYWxsb3dlZC1yb2xlcyI6WyJ1c2VyIl0sIngtaGFzdXJhLWRlZmF1bHQtcm9sZSI6InVzZXIiLCJ4LWhhc3VyYS11c2VyLWlkIjoiZDg0MzhlZDUtZmQ3Ny00NjNjLTk0MmUtNjY5YTc3NGRkNzc3In0sImlhdCI6MTcyMTM3MDY3NywiZXhwIjoxNzIzMzUxMjQ4NDg3LCJpc3MiOiJ6ZXJvLWFwcCJ9.jD4qiQjeSIppTdfI9MLtTsLHLpky-xPRG6kGbEeTnVA".to_string());

            KeyringState {
                storage: RwLock::new(initial_storage),
            }
        }
    }

    // Создаем глобальное состояние с помощью lazy_static
    lazy_static::lazy_static! {
        static ref STATE: Arc<KeyringState> = Arc::new(KeyringState {
            storage: RwLock::new(HashMap::new()),
        });
    }

    pub fn set(key: &str, value: &str) {
        let mut storage = STATE.storage.write().unwrap();
        storage.insert(key.to_string(), value.to_string());
    }

    pub fn get(key: &str) -> String {
        let storage = STATE.storage.read().unwrap();
        storage.get(key).cloned().unwrap_or_default()
    }

    pub fn delete(key: &str) {
        let mut storage = STATE.storage.write().unwrap();
        storage.remove(key);
    }
}
