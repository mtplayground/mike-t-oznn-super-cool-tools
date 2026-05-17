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
        <section class="grid gap-6 lg:grid-cols-[1.2fr_0.8fr]">
            <div class="flex flex-col gap-5">
                <p class="text-sm font-semibold uppercase tracking-[0.32em] text-cyan-300">
                    "Shell overview"
                </p>
                <h1 class="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                    "Hello toolbox"
                </h1>
                <p class="max-w-2xl text-base leading-7 text-slate-300">
                    "The shell now uses leptos_router with a persistent frame, category routes, tool routes, and a not-found view."
                </p>
                <div class="flex flex-wrap gap-3">
                    <A href="/category/math" attr:class="shell-pill">
                        "Browse math tools"
                    </A>
                    <A href="/tools/calculator" attr:class="shell-pill">
                        "Open tool placeholder"
                    </A>
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
                            <p class="text-sm text-slate-400">
                                "Loading runtime registry..."
                            </p>
                        }
                            .into_any(),
                        Some(Err(error)) => view! {
                            <p class="text-sm text-rose-300">{error.to_string()}</p>
                        }
                            .into_any(),
                        Some(Ok(catalog)) => {
                            let tool_count = catalog.tools().len();
                            let utility_count = catalog.by_tag("utility").len();
                            let matched = catalog.filter("calculator").len();

                            view! {
                                <div class="grid gap-3 sm:grid-cols-3">
                                    <div class="toolbox-panel p-4">
                                        <span class="shell-metric-label">"Loaded tools"</span>
                                        <strong class="shell-metric-value">{tool_count}</strong>
                                    </div>
                                    <div class="toolbox-panel p-4">
                                        <span class="shell-metric-label">"Utility tag matches"</span>
                                        <strong class="shell-metric-value">{utility_count}</strong>
                                    </div>
                                    <div class="toolbox-panel p-4">
                                        <span class="shell-metric-label">"Full-text \"calculator\""</span>
                                        <strong class="shell-metric-value">{matched}</strong>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    },
                }}
            </div>

            <aside class="toolbox-panel flex flex-col gap-4 p-6">
                <h2 class="text-lg font-semibold text-white">"Current routes"</h2>
                <ul class="space-y-3 text-sm text-slate-300">
                    <li><code class="text-cyan-200">"/"</code>" home shell landing page"</li>
                    <li><code class="text-cyan-200">"/category/:slug"</code>" category placeholder page"</li>
                    <li><code class="text-cyan-200">"/tools/:slug"</code>" tool placeholder slot"</li>
                    <li>"fallback route for unknown pages"</li>
                </ul>
            </aside>
        </section>
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
