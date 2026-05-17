#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use toolbox_core::{Category, ToolMeta, ToolRegistry};

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::{provide_context, use_context, LocalResource};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeRegistry {
    tools: Vec<ToolMeta>,
}

impl RuntimeRegistry {
    pub fn new(tools: Vec<ToolMeta>) -> Self {
        Self { tools }
    }

    pub fn from_tool_registry(registry: ToolRegistry) -> Self {
        Self::new(registry.tools.into_iter().map(|tool| tool.meta).collect())
    }

    pub fn tools(&self) -> &[ToolMeta] {
        &self.tools
    }

    pub fn by_category(&self, category: &Category) -> Vec<ToolMeta> {
        self.tools
            .iter()
            .filter(|tool| &tool.category == category)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    pub fn by_category_slug(&self, category_slug: &str) -> Vec<ToolMeta> {
        self.tools
            .iter()
            .filter(|tool| tool.category.as_str().eq_ignore_ascii_case(category_slug))
            .cloned()
            .collect()
    }

    pub fn by_slug(&self, slug: &str) -> Option<ToolMeta> {
        self.tools
            .iter()
            .find(|tool| tool.slug.eq_ignore_ascii_case(slug))
            .cloned()
    }

    pub fn by_tag(&self, tag: &str) -> Vec<ToolMeta> {
        self.tools
            .iter()
            .filter(|tool| tool.tags.iter().any(|tool_tag| tool_tag.eq_ignore_ascii_case(tag)))
            .cloned()
            .collect()
    }

    pub fn search_by_name_and_tags(&self, query: &str) -> Vec<ToolMeta> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        let query = trimmed.to_ascii_lowercase();
        self.tools
            .iter()
            .filter(|tool| {
                tool.name.to_ascii_lowercase().contains(&query)
                    || tool
                        .tags
                        .iter()
                        .any(|tag| tag.to_ascii_lowercase().contains(&query))
            })
            .cloned()
            .collect()
    }

    pub fn filter(&self, query: &str) -> Vec<ToolMeta> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return self.tools.clone();
        }

        let query = trimmed.to_ascii_lowercase();
        self.tools
            .iter()
            .filter(|tool| {
                tool.id.to_ascii_lowercase().contains(&query)
                    || tool.slug.to_ascii_lowercase().contains(&query)
                    || tool.name.to_ascii_lowercase().contains(&query)
                    || tool.description.to_ascii_lowercase().contains(&query)
                    || tool.category.as_str().contains(&query)
                    || tool
                        .tags
                        .iter()
                        .any(|tag| tag.to_ascii_lowercase().contains(&query))
            })
            .cloned()
            .collect()
    }
}

pub fn category_from_slug(slug: &str) -> Option<Category> {
    match slug.trim().to_ascii_lowercase().as_str() {
        "utilities" => Some(Category::Utilities),
        "math" => Some(Category::Math),
        "text" => Some(Category::Text),
        "developer" => Some(Category::Developer),
        "media" => Some(Category::Media),
        "productivity" => Some(Category::Productivity),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryLoadError {
    RequestFailed(String),
    HttpStatus { status: u16, body: String },
    ParseFailed(String),
}

#[cfg(target_arch = "wasm32")]
impl std::fmt::Display for RegistryLoadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestFailed(message) => {
                write!(formatter, "failed to fetch registry.json: {message}")
            }
            Self::HttpStatus { status, body } => {
                write!(formatter, "registry.json request returned status {status}: {body}")
            }
            Self::ParseFailed(message) => {
                write!(formatter, "failed to parse registry.json: {message}")
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl std::error::Error for RegistryLoadError {}

#[cfg(target_arch = "wasm32")]
pub type RegistryResource = LocalResource<Result<RuntimeRegistry, RegistryLoadError>>;

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct RegistryContext(pub RegistryResource);

#[cfg(target_arch = "wasm32")]
pub fn provide_registry_context() -> RegistryContext {
    let resource = LocalResource::new(load_registry);
    let context = RegistryContext(resource);
    provide_context(context.clone());
    context
}

#[cfg(target_arch = "wasm32")]
pub fn use_registry_context() -> Option<RegistryContext> {
    use_context::<RegistryContext>()
}

#[cfg(target_arch = "wasm32")]
async fn load_registry() -> Result<RuntimeRegistry, RegistryLoadError> {
    let response = Request::get("/registry.json")
        .send()
        .await
        .map_err(|error| RegistryLoadError::RequestFailed(error.to_string()))?;

    let status = response.status();
    if !(200..300).contains(&status) {
        let body = match response.text().await {
            Ok(body) => body,
            Err(error) => format!("failed to read error response body: {error}"),
        };
        return Err(RegistryLoadError::HttpStatus { status, body });
    }

    let registry = response
        .json::<ToolRegistry>()
        .await
        .map_err(|error| RegistryLoadError::ParseFailed(error.to_string()))?;

    Ok(RuntimeRegistry::from_tool_registry(registry))
}

#[cfg(test)]
mod tests {
    use super::RuntimeRegistry;
    use toolbox_core::{Category, ToolMeta};

    fn sample_registry() -> RuntimeRegistry {
        RuntimeRegistry::new(vec![
            ToolMeta::new(
                "calculator",
                "calculator",
                "Calculator",
                Category::Math,
                ["math", "utility"],
                "A calculator for arithmetic.",
                "/public/thumbnails/calculator.svg",
            ),
            ToolMeta::new(
                "json-pretty",
                "json-pretty",
                "JSON Pretty",
                Category::Developer,
                ["json", "formatting"],
                "Format and inspect JSON data.",
                "/assets/json-pretty.png",
            ),
        ])
    }

    #[test]
    fn selects_tools_by_category_slug() {
        let registry = sample_registry();
        let tools = registry.by_category_slug("math");

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].slug, "calculator");
    }

    #[test]
    fn selects_tools_by_slug() {
        let registry = sample_registry();
        let tool = registry.by_slug("json-pretty");

        assert_eq!(tool.map(|tool| tool.name), Some("JSON Pretty".to_owned()));
    }

    #[test]
    fn selects_tools_by_tag() {
        let registry = sample_registry();
        let tools = registry.by_tag("utility");

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].id, "calculator");
    }

    #[test]
    fn filters_tools_by_full_text() {
        let registry = sample_registry();
        let tools = registry.filter("inspect");

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].slug, "json-pretty");
    }

    #[test]
    fn searches_tools_by_name_and_tags_only() {
        let registry = sample_registry();

        let name_match = registry.search_by_name_and_tags("calc");
        let tag_match = registry.search_by_name_and_tags("format");
        let description_only_match = registry.search_by_name_and_tags("inspect");

        assert_eq!(name_match.len(), 1);
        assert_eq!(name_match[0].slug, "calculator");
        assert_eq!(tag_match.len(), 1);
        assert_eq!(tag_match[0].slug, "json-pretty");
        assert!(description_only_match.is_empty());
    }

    #[test]
    fn resolves_category_from_slug() {
        let category = super::category_from_slug("developer");

        assert_eq!(category, Some(Category::Developer));
    }
}
