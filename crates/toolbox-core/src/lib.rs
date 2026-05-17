#![forbid(unsafe_code)]

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn workspace_banner() -> &'static str {
    "toolbox-core placeholder crate"
}

#[cfg(test)]
mod tests {
    use super::{workspace_banner, CRATE_NAME};

    #[test]
    fn exposes_package_metadata() {
        assert_eq!(CRATE_NAME, "toolbox-core");
        assert_eq!(workspace_banner(), "toolbox-core placeholder crate");
    }
}
