use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{Category, RegistryError, ToolMeta};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRegistry {
    pub tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    pub fn parse_str(input: &str) -> Result<Self, RegistryError> {
        let registry = toml::from_str::<Self>(input)?;
        registry.validate()?;
        Ok(registry)
    }

    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| RegistryError::ReadFile {
            path: path.display().to_string(),
            source,
        })?;

        Self::parse_str(&contents)
    }

    pub fn validate(&self) -> Result<(), RegistryError> {
        let mut ids = HashSet::new();
        let mut slugs = HashSet::new();

        for tool in &self.tools {
            tool.validate()?;

            if !ids.insert(tool.meta.id.as_str()) {
                return Err(RegistryError::Invalid(format!(
                    "duplicate tool id `{}`",
                    tool.meta.id
                )));
            }

            if !slugs.insert(tool.meta.slug.as_str()) {
                return Err(RegistryError::Invalid(format!(
                    "duplicate tool slug `{}`",
                    tool.meta.slug
                )));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredTool {
    #[serde(flatten)]
    pub meta: ToolMeta,
    pub crate_path: String,
    pub entry_symbol: String,
    pub wasm_url: String,
}

impl RegisteredTool {
    pub fn validate(&self) -> Result<(), RegistryError> {
        validate_required("id", &self.meta.id)?;
        validate_required("slug", &self.meta.slug)?;
        validate_required("name", &self.meta.name)?;
        validate_required("description", &self.meta.description)?;
        validate_required("thumbnail", &self.meta.thumbnail)?;
        validate_required("crate_path", &self.crate_path)?;
        validate_required("entry_symbol", &self.entry_symbol)?;
        validate_required("wasm_url", &self.wasm_url)?;

        if self.meta.tags.is_empty() {
            return Err(RegistryError::Invalid(format!(
                "tool `{}` must declare at least one tag",
                self.meta.id
            )));
        }

        if self.meta.tags.iter().any(|tag| tag.trim().is_empty()) {
            return Err(RegistryError::Invalid(format!(
                "tool `{}` contains an empty tag",
                self.meta.id
            )));
        }

        Ok(())
    }

    pub fn category(&self) -> &Category {
        &self.meta.category
    }
}

fn validate_required(field: &str, value: &str) -> Result<(), RegistryError> {
    if value.trim().is_empty() {
        return Err(RegistryError::Invalid(format!(
            "field `{field}` must not be empty"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{RegisteredTool, ToolRegistry};
    use crate::Category;

    const VALID_REGISTRY: &str = r#"
        [[tools]]
        id = "calculator"
        slug = "calculator"
        name = "Calculator"
        category = "math"
        tags = ["math", "utility"]
        description = "A simple calculator."
        thumbnail = "/assets/calculator.png"
        crate_path = "crates/tools/calculator"
        entry_symbol = "mount"
        wasm_url = "/tools/calculator/calculator.wasm"
    "#;

    #[test]
    fn parses_valid_tool_registry() {
        let registry = ToolRegistry::parse_str(VALID_REGISTRY).expect("registry should parse");

        assert_eq!(registry.tools.len(), 1);
        let tool: &RegisteredTool = &registry.tools[0];
        assert_eq!(tool.meta.id, "calculator");
        assert_eq!(tool.meta.category, Category::Math);
        assert_eq!(tool.crate_path, "crates/tools/calculator");
        assert_eq!(tool.entry_symbol, "mount");
        assert_eq!(tool.wasm_url, "/tools/calculator/calculator.wasm");
    }

    #[test]
    fn rejects_duplicate_ids() {
        let registry = format!("{VALID_REGISTRY}\n{VALID_REGISTRY}");
        let error = ToolRegistry::parse_str(&registry).expect_err("duplicate ids should fail");

        assert!(matches!(error, crate::RegistryError::Invalid(message) if message.contains("duplicate tool id")));
    }

    #[test]
    fn rejects_empty_tags() {
        let registry = VALID_REGISTRY.replace(r#"tags = ["math", "utility"]"#, r#"tags = [""]"#);
        let error = ToolRegistry::parse_str(&registry).expect_err("empty tags should fail");

        assert!(matches!(error, crate::RegistryError::Invalid(message) if message.contains("empty tag")));
    }
}
