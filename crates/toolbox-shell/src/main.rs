#![forbid(unsafe_code)]

#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;

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
    view! {
        <main class="min-h-screen bg-slate-950 px-6 py-24 text-slate-100">
            <div class="mx-auto flex max-w-3xl flex-col gap-4 rounded-3xl border border-white/10 bg-white/5 p-10 shadow-2xl shadow-slate-950/40">
                <p class="text-sm font-medium uppercase tracking-[0.3em] text-cyan-300">
                    "Toolbox shell"
                </p>
                <h1 class="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                    "Hello toolbox"
                </h1>
                <p class="max-w-2xl text-base leading-7 text-slate-300">
                    "Trunk compiles this shell to WebAssembly and Tailwind provides the styling pipeline."
                </p>
            </div>
        </main>
    }
}
