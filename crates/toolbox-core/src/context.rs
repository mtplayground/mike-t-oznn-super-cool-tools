use crate::error::StorageError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    storage_namespace: String,
}

impl Context {
    pub fn new(storage_namespace: impl Into<String>) -> Self {
        Self {
            storage_namespace: storage_namespace.into(),
        }
    }

    pub fn storage_namespace(&self) -> &str {
        &self.storage_namespace
    }

    pub fn storage_key(&self, key: &str) -> String {
        format!("{}:{key}", self.storage_namespace)
    }

    pub fn get_local_storage_item(&self, key: &str) -> Result<Option<String>, StorageError> {
        let storage = storage()?;
        storage
            .get_item(&self.storage_key(key))
            .map_err(StorageError::StorageAccessFailed)
    }

    pub fn set_local_storage_item(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let storage = storage()?;
        storage
            .set_item(&self.storage_key(key), value)
            .map_err(StorageError::StorageAccessFailed)
    }

    pub fn remove_local_storage_item(&self, key: &str) -> Result<(), StorageError> {
        let storage = storage()?;
        storage
            .remove_item(&self.storage_key(key))
            .map_err(StorageError::StorageAccessFailed)
    }
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Result<web_sys::Storage, StorageError> {
    let window = web_sys::window().ok_or(StorageError::WindowUnavailable)?;
    let storage = window
        .local_storage()
        .map_err(StorageError::StorageAccessFailed)?;

    storage.ok_or(StorageError::LocalStorageUnavailable)
}

#[cfg(not(target_arch = "wasm32"))]
fn storage() -> Result<web_sys::Storage, StorageError> {
    Err(StorageError::UnsupportedPlatform)
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn prefixes_local_storage_keys() {
        let context = Context::new("toolbox");

        assert_eq!(context.storage_namespace(), "toolbox");
        assert_eq!(context.storage_key("recent"), "toolbox:recent");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_builds_return_storage_unavailable_errors() {
        let context = Context::new("toolbox");
        let error = context
            .get_local_storage_item("recent")
            .expect_err("native builds do not expose localStorage");

        assert!(matches!(
            error,
            crate::StorageError::UnsupportedPlatform
        ));
    }
}
