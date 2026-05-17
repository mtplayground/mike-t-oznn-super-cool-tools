#![cfg(target_arch = "wasm32")]

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use js_sys::{Function, Promise, Reflect};
use leptos::{html, prelude::*, task::spawn_local};
use toolbox_core::{
    TOOL_INIT_EXPORT_NAME, TOOL_LOADER_FILE_NAME, TOOL_MOUNT_EXPORT_NAME,
    TOOL_UNMOUNT_EXPORT_NAME,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlElement;

thread_local! {
    static TOOL_MODULE_CACHE: RefCell<HashMap<String, CachedToolModule>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
struct CachedToolModule {
    namespace: JsValue,
    initialized: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ToolHostStatus {
    Loading,
    Ready,
    Error(String),
}

#[component]
pub fn ToolHost(slug: String) -> impl IntoView {
    let container_ref = NodeRef::<html::Div>::new();
    let (status, set_status) = signal(ToolHostStatus::Loading);
    let (attempt, set_attempt) = signal(0_u32);
    let slug_for_effect = slug.clone();

    Effect::new(move |_| {
        let _ = attempt.get();
        let Some(container) = container_ref.get() else {
            return;
        };

        let active_slug = slug_for_effect.clone();
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_for_cleanup = Arc::clone(&cancelled);
        let slug_for_cleanup = active_slug.clone();

        set_status.set(ToolHostStatus::Loading);

        on_cleanup(move || {
            cancelled_for_cleanup.store(true, Ordering::SeqCst);
            let _ = unmount_tool(&slug_for_cleanup);
        });

        spawn_local({
            let set_status = set_status;
            async move {
                let result = load_and_mount_tool(&active_slug, container.into()).await;

                if cancelled.load(Ordering::SeqCst) {
                    return;
                }

                match result {
                    Ok(()) => set_status.set(ToolHostStatus::Ready),
                    Err(error) => set_status.set(ToolHostStatus::Error(error.to_string())),
                }
            }
        });
    });

    view! {
        <div class="tool-host">
            {move || match status.get() {
                ToolHostStatus::Loading => view! {
                    <div class="tool-host-skeleton" aria-busy="true" aria-live="polite">
                        <div class="tool-host-skeleton-bar tool-host-skeleton-bar-lg"></div>
                        <div class="tool-host-skeleton-bar"></div>
                        <div class="tool-host-skeleton-bar tool-host-skeleton-bar-sm"></div>
                    </div>
                }
                    .into_any(),
                ToolHostStatus::Ready => ().into_any(),
                ToolHostStatus::Error(message) => {
                    let retry_slug = slug.clone();
                    view! {
                        <div class="tool-host-error" role="alert">
                            <span class="tool-host-error-label">"Tool load failed"</span>
                            <p>{message}</p>
                            <button
                                type="button"
                                class="tool-host-retry"
                                on:click=move |_| {
                                    set_status.set(ToolHostStatus::Loading);
                                    clear_cached_tool_module(&retry_slug);
                                    set_attempt.update(|attempt| *attempt += 1);
                                }
                            >
                                "Retry loading tool"
                            </button>
                        </div>
                    }
                        .into_any()
                }
            }}

            <div node_ref=container_ref class="tool-host-container"></div>
        </div>
    }
}

async fn load_and_mount_tool(slug: &str, container: HtmlElement) -> Result<(), ToolHostError> {
    let namespace = load_tool_module(slug).await?;
    ensure_tool_initialized(slug, &namespace).await?;
    mount_tool(slug, &namespace, container)
}

async fn load_tool_module(slug: &str) -> Result<JsValue, ToolHostError> {
    if let Some(namespace) = TOOL_MODULE_CACHE.with(|cache| {
        cache
            .borrow()
            .get(slug)
            .map(|cached| cached.namespace.clone())
    }) {
        return Ok(namespace);
    }

    let module_path = tool_loader_path(slug);
    let promise = import_tool_module(&module_path)
        .map_err(|error| ToolHostError::ImportFailed(module_path.clone(), js_error_message(error)))?;
    let namespace = JsFuture::from(promise)
        .await
        .map_err(|error| ToolHostError::ImportFailed(module_path, js_error_message(error)))?;

    TOOL_MODULE_CACHE.with(|cache| {
        cache.borrow_mut().insert(
            slug.to_owned(),
            CachedToolModule {
                namespace: namespace.clone(),
                initialized: false,
            },
        );
    });

    Ok(namespace)
}

async fn ensure_tool_initialized(slug: &str, namespace: &JsValue) -> Result<(), ToolHostError> {
    if TOOL_MODULE_CACHE.with(|cache| {
        cache
            .borrow()
            .get(slug)
            .map(|cached| cached.initialized)
            .unwrap_or(false)
    }) {
        return Ok(());
    }

    let init = export_function(namespace, TOOL_INIT_EXPORT_NAME, slug)?;
    let promise = init
        .call0(namespace)
        .map_err(|error| ToolHostError::InitFailed(slug.to_owned(), js_error_message(error)))?
        .dyn_into::<Promise>()
        .map_err(|_| {
            ToolHostError::InitFailed(
                slug.to_owned(),
                "loader default export did not return a Promise".to_owned(),
            )
        })?;

    JsFuture::from(promise)
        .await
        .map_err(|error| ToolHostError::InitFailed(slug.to_owned(), js_error_message(error)))?;

    TOOL_MODULE_CACHE.with(|cache| {
        if let Some(cached) = cache.borrow_mut().get_mut(slug) {
            cached.initialized = true;
        }
    });

    Ok(())
}

fn mount_tool(
    slug: &str,
    namespace: &JsValue,
    container: HtmlElement,
) -> Result<(), ToolHostError> {
    let mount = export_function(namespace, TOOL_MOUNT_EXPORT_NAME, slug)?;
    mount
        .call1(namespace, &container.into())
        .map_err(|error| ToolHostError::MountFailed(slug.to_owned(), js_error_message(error)))?;
    Ok(())
}

fn unmount_tool(slug: &str) -> Result<(), ToolHostError> {
    let namespace = TOOL_MODULE_CACHE.with(|cache| {
        cache
            .borrow()
            .get(slug)
            .map(|cached| cached.namespace.clone())
    });

    let Some(namespace) = namespace else {
        return Ok(());
    };

    let unmount = export_function(&namespace, TOOL_UNMOUNT_EXPORT_NAME, slug)?;
    unmount
        .call0(&namespace)
        .map_err(|error| ToolHostError::UnmountFailed(slug.to_owned(), js_error_message(error)))?;
    Ok(())
}

fn export_function(
    namespace: &JsValue,
    export_name: &str,
    slug: &str,
) -> Result<Function, ToolHostError> {
    let export = Reflect::get(namespace, &JsValue::from_str(export_name)).map_err(|error| {
        ToolHostError::InvalidModuleExport(
            slug.to_owned(),
            export_name.to_owned(),
            js_error_message(error),
        )
    })?;

    export.dyn_into::<Function>().map_err(|_| {
        ToolHostError::InvalidModuleExport(
            slug.to_owned(),
            export_name.to_owned(),
            "export is not callable".to_owned(),
        )
    })
}

fn tool_loader_path(slug: &str) -> String {
    format!("/tools/{slug}/{TOOL_LOADER_FILE_NAME}")
}

fn clear_cached_tool_module(slug: &str) {
    TOOL_MODULE_CACHE.with(|cache| {
        cache.borrow_mut().remove(slug);
    });
}

fn js_error_message(error: JsValue) -> String {
    error
        .as_string()
        .or_else(|| js_sys::Error::from(error).message().as_string())
        .unwrap_or_else(|| "unknown JavaScript error".to_owned())
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ToolHostError {
    ImportFailed(String, String),
    InvalidModuleExport(String, String, String),
    InitFailed(String, String),
    MountFailed(String, String),
    UnmountFailed(String, String),
}

impl std::fmt::Display for ToolHostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImportFailed(path, message) => {
                write!(formatter, "failed to import `{path}`: {message}")
            }
            Self::InvalidModuleExport(slug, export_name, message) => write!(
                formatter,
                "tool `{slug}` exported invalid `{export_name}` loader binding: {message}"
            ),
            Self::InitFailed(slug, message) => {
                write!(formatter, "tool `{slug}` failed to initialize: {message}")
            }
            Self::MountFailed(slug, message) => {
                write!(formatter, "tool `{slug}` failed to mount: {message}")
            }
            Self::UnmountFailed(slug, message) => {
                write!(formatter, "tool `{slug}` failed to unmount: {message}")
            }
        }
    }
}

impl std::error::Error for ToolHostError {}

#[wasm_bindgen(inline_js = "
export function importToolModule(path) {
  return import(path);
}
")]
extern "C" {
    #[wasm_bindgen(catch, js_name = importToolModule)]
    fn import_tool_module(path: &str) -> Result<Promise, JsValue>;
}

#[cfg(test)]
mod tests {
    use super::tool_loader_path;

    #[test]
    fn builds_loader_path_from_slug() {
        assert_eq!(tool_loader_path("calculator"), "/tools/calculator/loader.js");
    }
}
