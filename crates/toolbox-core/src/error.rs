use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool mount failed: {0}")]
    Mount(String),
    #[error("tool unmount failed: {0}")]
    Unmount(String),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("window is unavailable")]
    WindowUnavailable,
    #[error("localStorage is unavailable")]
    LocalStorageUnavailable,
    #[error("localStorage access failed")]
    StorageAccessFailed(web_sys::wasm_bindgen::JsValue),
    #[error("localStorage is only available on wasm32 targets")]
    UnsupportedPlatform,
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("failed to read registry file `{path}`: {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse registry TOML: {0}")]
    ParseToml(#[from] toml::de::Error),
    #[error("invalid tool registry: {0}")]
    Invalid(String),
}
