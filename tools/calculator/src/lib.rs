#![forbid(unsafe_code)]

use wasm_bindgen::prelude::wasm_bindgen;

pub const TOOL_NAME: &str = "calculator";

pub fn display_name() -> &'static str {
    "Calculator"
}

#[wasm_bindgen]
pub fn mount_calculator() -> String {
    display_name().to_owned()
}

#[cfg(test)]
mod tests {
    use super::{display_name, mount_calculator, TOOL_NAME};

    #[test]
    fn exposes_tool_identity() {
        assert_eq!(TOOL_NAME, "calculator");
        assert_eq!(display_name(), "Calculator");
        assert_eq!(mount_calculator(), "Calculator");
    }
}
