#![forbid(unsafe_code)]

pub const TOOL_NAME: &str = "calculator";

pub fn display_name() -> &'static str {
    "Calculator"
}

#[cfg(test)]
mod tests {
    use super::{display_name, TOOL_NAME};

    #[test]
    fn exposes_tool_identity() {
        assert_eq!(TOOL_NAME, "calculator");
        assert_eq!(display_name(), "Calculator");
    }
}
