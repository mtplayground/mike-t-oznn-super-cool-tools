#![forbid(unsafe_code)]

mod registry;

#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::{use_location, use_params_map},
    path,
};
#[cfg(target_arch = "wasm32")]
use registry::{category_from_slug, provide_registry_context, use_registry_context};
#[cfg(target_arch = "wasm32")]
use toolbox_core::{Category, ToolMeta};

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("toolbox-shell is intended to run in the browser via trunk.");
}

#[cfg(target_arch = "wasm32")]
#[component]
fn App() -> impl IntoView {
    let _registry = provide_registry_context();

    view! {
        <Router>
            <div class="shell-page">
                <SiteHeader />
                <main class="mx-auto flex w-full max-w-6xl flex-1 flex-col gap-8 px-6 pb-12 pt-8 sm:px-8 lg:px-10">
                    <Breadcrumbs />
                    <section class="toolbox-panel shell-panel">
                        <Routes fallback=NotFoundPage>
                            <Route path=path!("/") view=HomePage />
                            <Route path=path!("/category/:slug") view=CategoryPage />
                            <Route path=path!("/tools/:slug") view=ToolPage />
                        </Routes>
                    </section>
                </main>
                <SiteFooter />
            </div>
        </Router>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header class="border-b border-white/10 bg-slate-950/70 backdrop-blur-xl">
            <div class="mx-auto flex w-full max-w-6xl items-center justify-between gap-6 px-6 py-5 sm:px-8 lg:px-10">
                <A href="/" attr:class="flex min-w-0 flex-col">
                    <span class="text-xs font-semibold uppercase tracking-[0.32em] text-cyan-300">
                        "Mike T Oznn"
                    </span>
                    <span class="text-lg font-semibold tracking-tight text-white">
                        "Super Cool Tools"
                    </span>
                </A>

                <nav class="flex items-center gap-3 text-sm text-slate-300">
                    <A href="/" attr:class="shell-nav-link">
                        "Home"
                    </A>
                    <A href="/category/utilities" attr:class="shell-nav-link">
                        "Utilities"
                    </A>
                    <A href="/tools/calculator" attr:class="shell-nav-link">
                        "Tool Slot"
                    </A>
                </nav>
            </div>
        </header>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn Breadcrumbs() -> impl IntoView {
    let location = use_location();

    let crumbs = move || {
        let pathname = location.pathname.get();
        let trimmed = pathname.trim_matches('/');

        if trimmed.is_empty() {
            return vec![BreadcrumbItem {
                label: "Home".to_owned(),
                href: "/".to_owned(),
                current: true,
            }];
        }

        let mut parts = Vec::new();
        parts.push(BreadcrumbItem {
            label: "Home".to_owned(),
            href: "/".to_owned(),
            current: false,
        });

        let mut href = String::new();
        let segments: Vec<_> = trimmed.split('/').collect();
        for (index, segment) in segments.iter().enumerate() {
            href.push('/');
            href.push_str(segment);
            parts.push(BreadcrumbItem {
                label: format_segment(segment),
                href: href.clone(),
                current: index + 1 == segments.len(),
            });
        }

        parts
    };

    view! {
        <nav aria-label="Breadcrumb" class="flex flex-wrap items-center gap-2 text-sm text-slate-400">
            <For
                each=crumbs
                key=|crumb| crumb.href.clone()
                children=move |crumb| {
                    if crumb.current {
                        view! {
                            <>
                                <span class="text-slate-600" aria-hidden="true">
                                    "/"
                                </span>
                                <span class="font-medium text-slate-200">{crumb.label}</span>
                            </>
                        }
                            .into_any()
                    } else {
                        view! {
                            <>
                                <span class="text-slate-600" aria-hidden="true">
                                    "/"
                                </span>
                                <A href=crumb.href attr:class="transition hover:text-cyan-300">
                                    {crumb.label}
                                </A>
                            </>
                        }
                            .into_any()
                    }
                }
            />
        </nav>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn HomePage() -> impl IntoView {
    let registry = use_registry_context();

    view! {
        <section class="flex flex-col gap-8">
            <div class="flex flex-col gap-5">
                <p class="text-sm font-semibold uppercase tracking-[0.32em] text-cyan-300">
                    "Tool index"
                </p>
                <h1 class="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                    "Browse every tool by category"
                </h1>
                <p class="max-w-2xl text-base leading-7 text-slate-300">
                    "The index page groups registered tools by category and links each card directly to its tool route."
                </p>
                <div class="flex flex-wrap gap-3">
                    <A href="/category/math" attr:class="shell-pill">
                        "Browse math tools"
                    </A>
                    <A href="/tools/calculator" attr:class="shell-pill">
                        "Open calculator"
                    </A>
                </div>
            </div>

            {move || match registry.clone() {
                None => view! {
                    <p class="text-sm text-amber-200">
                        "Registry context is unavailable."
                    </p>
                }
                    .into_any(),
                Some(registry) => match registry.0.get() {
                    None => view! {
                        <div class="toolbox-panel flex min-h-[16rem] items-center justify-center p-8">
                            <p class="text-sm text-slate-400">
                                "Loading runtime registry..."
                            </p>
                        </div>
                    }
                        .into_any(),
                    Some(Err(error)) => view! {
                        <div class="toolbox-panel flex min-h-[16rem] items-center justify-center p-8">
                            <p class="text-sm text-rose-300">{error.to_string()}</p>
                        </div>
                    }
                        .into_any(),
                    Some(Ok(catalog)) => {
                        let total_tools = catalog.tools().len();
                        let category_count = all_categories().len();

                        view! {
                            <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
                                <div class="toolbox-panel p-4">
                                    <span class="shell-metric-label">"Loaded tools"</span>
                                    <strong class="shell-metric-value">{total_tools}</strong>
                                </div>
                                <div class="toolbox-panel p-4">
                                    <span class="shell-metric-label">"Categories"</span>
                                    <strong class="shell-metric-value">{category_count}</strong>
                                </div>
                                <div class="toolbox-panel p-4">
                                    <span class="shell-metric-label">"Utility tag matches"</span>
                                    <strong class="shell-metric-value">{catalog.by_tag("utility").len()}</strong>
                                </div>
                                <div class="toolbox-panel p-4">
                                    <span class="shell-metric-label">"Full-text \"calculator\""</span>
                                    <strong class="shell-metric-value">{catalog.filter("calculator").len()}</strong>
                                </div>
                            </div>

                            <div class="flex flex-col gap-8">
                                <For
                                    each=all_categories
                                    key=|category| category.as_str().to_owned()
                                    children=move |category| {
                                        let tools = catalog.by_category(&category);
                                        view! {
                                            <CategorySection category=category tools=tools />
                                        }
                                    }
                                />
                            </div>
                        }
                            .into_any()
                    }
                },
            }}
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn CategorySection(category: Category, tools: Vec<ToolMeta>) -> impl IntoView {
    let category_label = category.label().to_owned();
    let category_slug = category.as_str().to_owned();
    let tool_count = tools.len();
    let has_tools = tool_count > 0;

    view! {
        <section class="flex flex-col gap-5">
            <div class="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
                <div class="flex flex-col gap-2">
                    <p class="text-xs font-semibold uppercase tracking-[0.28em] text-slate-400">
                        {format!("{} tool{}", tool_count, if tool_count == 1 { "" } else { "s" })}
                    </p>
                    <h2 class="text-2xl font-semibold tracking-tight text-white sm:text-3xl">
                        {category_label.clone()}
                    </h2>
                </div>

                <A
                    href=format!("/category/{category_slug}")
                    attr:class="text-sm font-medium text-cyan-200 transition hover:text-cyan-100"
                >
                    "Open category"
                </A>
            </div>

            {if has_tools {
                view! {
                    <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
                        <For
                            each=move || tools.clone()
                            key=|tool| tool.slug.clone()
                            children=move |tool| {
                                view! { <ToolCard tool=tool /> }
                            }
                        />
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div class="rounded-3xl border border-dashed border-white/10 bg-slate-900/20 px-5 py-8 text-sm text-slate-400">
                        {format!("No {} tools are registered yet.", category_label.to_ascii_lowercase())}
                    </div>
                }
                    .into_any()
            }}
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn ToolCard(tool: ToolMeta) -> impl IntoView {
    let tool_link = format!("/tools/{}", tool.slug);
    let thumbnail_alt = format!("{} thumbnail", tool.name);
    let short_description = short_description(&tool.description, 88);
    let visible_tags = tool.tags.iter().take(3).cloned().collect::<Vec<_>>();

    view! {
        <A href=tool_link attr:class="tool-card group">
            <div class="tool-card-thumbnail">
                <img
                    src=tool.thumbnail.clone()
                    alt=thumbnail_alt
                    class="h-full w-full object-cover"
                    loading="lazy"
                />
                <div class="tool-card-thumbnail-overlay">
                    <span class="tool-card-category">{tool.category.label()}</span>
                </div>
            </div>

            <div class="flex flex-1 flex-col gap-3 p-5">
                <div class="flex items-start justify-between gap-4">
                    <div class="min-w-0">
                        <h3 class="text-lg font-semibold tracking-tight text-white transition group-hover:text-cyan-100">
                            {tool.name.clone()}
                        </h3>
                        <p class="mt-1 text-sm text-slate-400">{short_description}</p>
                    </div>
                    <span class="tool-card-arrow" aria-hidden="true">
                        "↗"
                    </span>
                </div>

                <div class="mt-auto flex flex-wrap gap-2">
                    <For
                        each=move || visible_tags.clone()
                        key=|tag| tag.clone()
                        children=move |tag| {
                            view! {
                                <span class="tool-card-tag">{tag}</span>
                            }
                        }
                    />
                </div>
            </div>
        </A>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn CategoryPage() -> impl IntoView {
    let params = use_params_map();
    let registry = use_registry_context();
    let slug = move || {
        params
            .with(|params| params.get("slug"))
            .unwrap_or_else(|| "unknown".to_owned())
    };

    view! {
        <section class="flex flex-col gap-6">
            <div class="flex flex-col gap-3">
                <p class="text-sm font-semibold uppercase tracking-[0.32em] text-cyan-300">
                    "Category route"
                </p>
                <h1 class="text-4xl font-semibold tracking-tight text-white">
                    {move || format!("Category: {}", format_segment(&slug()))}
                </h1>
                <p class="max-w-2xl text-base leading-7 text-slate-300">
                    "This route is wired and ready for issue #9 to load category-specific tool groups."
                </p>
            </div>

            <div class="toolbox-panel flex flex-col gap-3 p-6">
                <span class="text-xs uppercase tracking-[0.28em] text-slate-400">"Slug"</span>
                <code class="text-sm text-cyan-200">{slug}</code>
            </div>

            {move || match registry.clone() {
                None => view! {
                    <p class="text-sm text-amber-200">"Registry context is unavailable."</p>
                }
                    .into_any(),
                Some(registry) => match registry.0.get() {
                    None => view! {
                        <p class="text-sm text-slate-400">"Loading category tools..."</p>
                    }
                        .into_any(),
                    Some(Err(error)) => view! {
                        <p class="text-sm text-rose-300">{error.to_string()}</p>
                    }
                        .into_any(),
                    Some(Ok(catalog)) => {
                        let current_slug = slug();
                        let tools = category_from_slug(&current_slug)
                            .map(|category| catalog.by_category(&category))
                            .unwrap_or_else(|| catalog.by_category_slug(&current_slug));
                        let has_tools = !tools.is_empty();

                        view! {
                            <div class="toolbox-panel flex flex-col gap-4 p-6">
                                <div class="flex items-center justify-between gap-4">
                                    <h2 class="text-lg font-semibold text-white">"Registered tools"</h2>
                                    <span class="text-sm text-slate-400">
                                        {format!("{} match(es)", tools.len())}
                                    </span>
                                </div>

                                {if has_tools {
                                    view! {
                                        <ul class="space-y-3">
                                            <For
                                                each=move || tools.clone()
                                                key=|tool| tool.slug.clone()
                                                children=move |tool| {
                                                    view! {
                                                        <li class="rounded-2xl border border-white/10 bg-slate-900/40 px-4 py-3">
                                                            <A href=format!("/tools/{}", tool.slug) attr:class="font-medium text-cyan-200 hover:text-cyan-100">
                                                                {tool.name}
                                                            </A>
                                                            <p class="mt-1 text-sm text-slate-400">{tool.description}</p>
                                                        </li>
                                                    }
                                                }
                                            />
                                        </ul>
                                    }.into_any()
                                } else {
                                    view! {
                                        <p class="text-sm text-slate-400">
                                            "No tools in this category have been loaded yet."
                                        </p>
                                    }.into_any()
                                }}
                            </div>
                        }.into_any()
                    }
                },
            }}
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn ToolPage() -> impl IntoView {
    let params = use_params_map();
    let registry = use_registry_context();
    let slug = move || {
        params
            .with(|params| params.get("slug"))
            .unwrap_or_else(|| "unknown".to_owned())
    };

    view! {
        <section class="grid gap-6 lg:grid-cols-[0.9fr_1.1fr]">
            <div class="flex flex-col gap-4">
                <p class="text-sm font-semibold uppercase tracking-[0.32em] text-cyan-300">
                    "Tool route"
                </p>
                <h1 class="text-4xl font-semibold tracking-tight text-white">
                    {move || format!("Tool: {}", format_segment(&slug()))}
                </h1>
                <p class="max-w-xl text-base leading-7 text-slate-300">
                    "This route reserves the content slot where the tool host will mount the selected tool in a later issue."
                </p>
            </div>

            <div class="shell-placeholder">
                {move || match registry.clone() {
                    None => view! {
                        <p class="text-sm text-amber-200">"Registry context is unavailable."</p>
                    }
                        .into_any(),
                    Some(registry) => match registry.0.get() {
                        None => view! {
                            <div class="flex flex-col gap-3">
                                <span class="text-xs uppercase tracking-[0.28em] text-slate-400">
                                    "Placeholder slot"
                                </span>
                                <strong class="text-xl font-semibold text-white">
                                    {format!("{} host surface", format_segment(&slug()))}
                                </strong>
                                <p class="text-sm leading-6 text-slate-300">
                                    "Loading tool metadata from registry.json..."
                                </p>
                            </div>
                        }
                            .into_any(),
                        Some(Err(error)) => view! {
                            <div class="flex flex-col gap-3">
                                <span class="text-xs uppercase tracking-[0.28em] text-rose-300">
                                    "Registry error"
                                </span>
                                <p class="text-sm leading-6 text-rose-200">{error.to_string()}</p>
                            </div>
                        }
                            .into_any(),
                        Some(Ok(catalog)) => match catalog.by_slug(&slug()) {
                            Some(tool) => view! {
                                <div class="flex flex-col gap-3">
                                    <span class="text-xs uppercase tracking-[0.28em] text-slate-400">
                                        "Placeholder slot"
                                    </span>
                                    <strong class="text-xl font-semibold text-white">
                                        {format!("{} host surface", tool.name)}
                                    </strong>
                                    <p class="text-sm leading-6 text-slate-300">{tool.description}</p>
                                    <div class="flex flex-wrap gap-2">
                                        <For
                                            each=move || tool.tags.clone()
                                            key=|tag| tag.clone()
                                            children=move |tag| {
                                                view! {
                                                    <span class="rounded-full border border-cyan-400/20 bg-cyan-400/10 px-3 py-1 text-xs font-medium uppercase tracking-[0.2em] text-cyan-100">
                                                        {tag}
                                                    </span>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            }
                                .into_any(),
                            None => view! {
                                <div class="flex flex-col gap-3">
                                    <span class="text-xs uppercase tracking-[0.28em] text-slate-400">
                                        "Placeholder slot"
                                    </span>
                                    <strong class="text-xl font-semibold text-white">
                                        {format!("{} host surface", format_segment(&slug()))}
                                    </strong>
                                    <p class="text-sm leading-6 text-slate-300">
                                        "No registry entry matched this tool slug."
                                    </p>
                                </div>
                            }
                                .into_any(),
                        },
                    },
                }}
            </div>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="flex min-h-[28rem] flex-col items-start justify-center gap-5">
            <span class="text-sm font-semibold uppercase tracking-[0.32em] text-rose-300">
                "404"
            </span>
            <h1 class="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                "This route does not exist"
            </h1>
            <p class="max-w-xl text-base leading-7 text-slate-300">
                "The shell keeps its shared frame even when a route misses, so navigation stays consistent."
            </p>
            <A href="/" attr:class="shell-pill">
                "Return home"
            </A>
        </section>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="border-t border-white/10">
            <div class="mx-auto flex w-full max-w-6xl flex-col gap-3 px-6 py-6 text-sm text-slate-400 sm:px-8 lg:flex-row lg:items-center lg:justify-between lg:px-10">
                <p>"Persistent shell layout for navigation, breadcrumbs, and route placeholders."</p>
                <p class="font-medium text-slate-300">"Leptos Router + Trunk + Tailwind"</p>
            </div>
        </footer>
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, Eq)]
struct BreadcrumbItem {
    label: String,
    href: String,
    current: bool,
}

#[cfg(target_arch = "wasm32")]
fn format_segment(segment: &str) -> String {
    segment
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut label = first.to_uppercase().collect::<String>();
                    label.push_str(chars.as_str());
                    label
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(target_arch = "wasm32")]
fn all_categories() -> Vec<Category> {
    vec![
        Category::Utilities,
        Category::Math,
        Category::Text,
        Category::Developer,
        Category::Media,
        Category::Productivity,
    ]
}

#[cfg(target_arch = "wasm32")]
fn short_description(description: &str, max_chars: usize) -> String {
    let trimmed = description.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_owned();
    }

    let mut shortened = trimmed.chars().take(max_chars.saturating_sub(1)).collect::<String>();
    shortened.push('…');
    shortened
}
