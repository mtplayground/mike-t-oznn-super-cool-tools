use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    #[default]
    Utilities,
    Math,
    Text,
    Developer,
    Media,
    Productivity,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Utilities => "utilities",
            Self::Math => "math",
            Self::Text => "text",
            Self::Developer => "developer",
            Self::Media => "media",
            Self::Productivity => "productivity",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Utilities => "Utilities",
            Self::Math => "Math",
            Self::Text => "Text",
            Self::Developer => "Developer",
            Self::Media => "Media",
            Self::Productivity => "Productivity",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolMeta {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub category: Category,
    pub tags: Vec<String>,
    pub description: String,
    pub thumbnail: String,
}

impl ToolMeta {
    pub fn new(
        id: impl Into<String>,
        slug: impl Into<String>,
        name: impl Into<String>,
        category: Category,
        tags: impl IntoIterator<Item = impl Into<String>>,
        description: impl Into<String>,
        thumbnail: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            slug: slug.into(),
            name: name.into(),
            category,
            tags: tags.into_iter().map(Into::into).collect(),
            description: description.into(),
            thumbnail: thumbnail.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Category, ToolMeta};

    #[test]
    fn category_string_values_are_stable() {
        assert_eq!(Category::Utilities.as_str(), "utilities");
        assert_eq!(Category::Developer.label(), "Developer");
    }

    #[test]
    fn tool_meta_collects_owned_values() {
        let meta = ToolMeta::new(
            "calculator",
            "calculator",
            "Calculator",
            Category::Math,
            ["math", "utility"],
            "A simple calculator.",
            "/assets/calculator.png",
        );

        assert_eq!(meta.id, "calculator");
        assert_eq!(meta.slug, "calculator");
        assert_eq!(meta.name, "Calculator");
        assert_eq!(meta.category, Category::Math);
        assert_eq!(meta.tags, vec!["math", "utility"]);
        assert_eq!(meta.description, "A simple calculator.");
        assert_eq!(meta.thumbnail, "/assets/calculator.png");
    }
}
