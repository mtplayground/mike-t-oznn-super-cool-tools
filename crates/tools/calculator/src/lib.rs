#![forbid(unsafe_code)]

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;

pub const TOOL_NAME: &str = "calculator";

thread_local! {
    static LAST_HOST: std::cell::RefCell<Option<HtmlElement>> = const { std::cell::RefCell::new(None) };
}

pub fn display_name() -> &'static str {
    "Calculator"
}

#[wasm_bindgen]
pub fn mount(host_element: HtmlElement) {
    host_element.set_inner_html(
        r#"
        <div class="flex flex-col gap-3 rounded-3xl border border-cyan-400/20 bg-slate-900/70 p-6 text-slate-100">
          <span class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-300">Calculator</span>
          <strong class="text-2xl font-semibold tracking-tight text-white">Calculator</strong>
          <p class="text-sm leading-6 text-slate-300">Stub UI rendered into the host element through the loader contract.</p>
        </div>
        "#,
    );

    LAST_HOST.with(|last_host| {
        last_host.replace(Some(host_element));
    });
}

#[wasm_bindgen]
pub fn unmount() {
    LAST_HOST.with(|last_host| {
        if let Some(host) = last_host.borrow_mut().take() {
            host.set_inner_html("");
        }
    });
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
