#![forbid(unsafe_code)]

mod state;

use std::cell::RefCell;
use std::rc::Rc;

use state::{action_from_key, action_from_token, CalculatorState};
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{Element, Event, HtmlElement, KeyboardEvent, Window};

pub use state::{Action, Operator};

pub const TOOL_NAME: &str = "calculator";

thread_local! {
    static MOUNTED_CALCULATOR: RefCell<Option<MountedCalculator>> = const { RefCell::new(None) };
}

struct MountedCalculator {
    host: HtmlElement,
    window: Window,
    click_listener: Closure<dyn FnMut(Event)>,
    keyboard_listener: Closure<dyn FnMut(KeyboardEvent)>,
}

pub fn display_name() -> &'static str {
    "Calculator"
}

#[wasm_bindgen]
pub fn mount(host_element: HtmlElement) -> Result<(), JsValue> {
    unmount()?;

    render_markup(&host_element);

    let display = query_html_element(&host_element, "[data-role='display']")?;
    let clear_button = query_html_element(&host_element, "[data-role='clear']")?;
    let state = Rc::new(RefCell::new(CalculatorState::new()));
    sync_view(&state, &display, &clear_button);

    let click_state = Rc::clone(&state);
    let click_display = display.clone();
    let click_clear = clear_button.clone();
    let click_listener = Closure::wrap(Box::new(move |event: Event| {
        if let Some(action) = event_action(&event) {
            click_state.borrow_mut().apply(action);
            sync_view(&click_state, &click_display, &click_clear);
        }
    }) as Box<dyn FnMut(_)>);

    host_element
        .add_event_listener_with_callback("click", click_listener.as_ref().unchecked_ref())
        .map_err(JsValue::from)?;

    let window = web_sys::window().ok_or_else(|| js_error("window is unavailable"))?;
    let keyboard_state = Rc::clone(&state);
    let keyboard_display = display.clone();
    let keyboard_clear = clear_button.clone();
    let keyboard_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(action) = action_from_key(&event.key()) {
            event.prevent_default();
            keyboard_state.borrow_mut().apply(action);
            sync_view(&keyboard_state, &keyboard_display, &keyboard_clear);
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keydown", keyboard_listener.as_ref().unchecked_ref())
        .map_err(JsValue::from)?;

    MOUNTED_CALCULATOR.with(|mounted| {
        mounted.replace(Some(MountedCalculator {
            host: host_element,
            window,
            click_listener,
            keyboard_listener,
        }));
    });

    Ok(())
}

#[wasm_bindgen]
pub fn unmount() -> Result<(), JsValue> {
    MOUNTED_CALCULATOR.with(|mounted| {
        let Some(mounted) = mounted.borrow_mut().take() else {
            return Ok(());
        };

        mounted
            .host
            .remove_event_listener_with_callback("click", mounted.click_listener.as_ref().unchecked_ref())
            .map_err(JsValue::from)?;
        mounted
            .window
            .remove_event_listener_with_callback(
                "keydown",
                mounted.keyboard_listener.as_ref().unchecked_ref(),
            )
            .map_err(JsValue::from)?;
        mounted.host.set_inner_html("");

        Ok(())
    })
}

fn render_markup(host: &HtmlElement) {
    host.set_inner_html(
        r#"
        <section class="flex flex-col gap-4 rounded-[1.75rem] border border-cyan-400/20 bg-slate-950/80 p-5 shadow-[0_30px_80px_rgba(15,23,42,0.45)]">
          <div class="flex items-center justify-between gap-4">
            <div>
              <span class="text-xs font-semibold uppercase tracking-[0.26em] text-cyan-300">Calculator</span>
              <p class="mt-2 text-sm text-slate-400">Four-function arithmetic with keyboard support.</p>
            </div>
            <div class="rounded-2xl border border-white/10 bg-slate-900/80 px-4 py-3 text-right">
              <div class="text-xs uppercase tracking-[0.22em] text-slate-500">Display</div>
              <div data-role="display" class="mt-2 min-w-[10rem] text-3xl font-semibold tracking-tight text-white">0</div>
            </div>
          </div>

          <div class="grid grid-cols-4 gap-3">
            <button type="button" data-role="clear" data-input="clear" class="rounded-2xl border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm font-semibold text-rose-100 transition hover:bg-rose-400/20">AC</button>
            <button type="button" data-input="backspace" class="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-white/10">⌫</button>
            <button type="button" data-input="divide" class="rounded-2xl border border-cyan-400/25 bg-cyan-400/10 px-4 py-3 text-sm font-semibold text-cyan-100 transition hover:bg-cyan-400/20">÷</button>
            <button type="button" data-input="multiply" class="rounded-2xl border border-cyan-400/25 bg-cyan-400/10 px-4 py-3 text-sm font-semibold text-cyan-100 transition hover:bg-cyan-400/20">×</button>

            <button type="button" data-input="digit-7" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">7</button>
            <button type="button" data-input="digit-8" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">8</button>
            <button type="button" data-input="digit-9" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">9</button>
            <button type="button" data-input="subtract" class="rounded-2xl border border-cyan-400/25 bg-cyan-400/10 px-4 py-3 text-sm font-semibold text-cyan-100 transition hover:bg-cyan-400/20">−</button>

            <button type="button" data-input="digit-4" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">4</button>
            <button type="button" data-input="digit-5" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">5</button>
            <button type="button" data-input="digit-6" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">6</button>
            <button type="button" data-input="add" class="rounded-2xl border border-cyan-400/25 bg-cyan-400/10 px-4 py-3 text-sm font-semibold text-cyan-100 transition hover:bg-cyan-400/20">+</button>

            <button type="button" data-input="digit-1" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">1</button>
            <button type="button" data-input="digit-2" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">2</button>
            <button type="button" data-input="digit-3" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">3</button>
            <button type="button" data-input="equals" class="row-span-2 rounded-[1.5rem] border border-emerald-400/25 bg-emerald-400/15 px-4 py-3 text-sm font-semibold text-emerald-100 transition hover:bg-emerald-400/25">=</button>

            <button type="button" data-input="digit-0" class="col-span-2 rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">0</button>
            <button type="button" data-input="decimal" class="rounded-2xl border border-white/10 bg-slate-900/90 px-4 py-3 text-sm font-semibold text-slate-100 transition hover:bg-slate-800">.</button>
          </div>
        </section>
        "#,
    );
}

fn query_html_element(host: &HtmlElement, selector: &str) -> Result<HtmlElement, JsValue> {
    let element = host
        .query_selector(selector)
        .map_err(JsValue::from)?
        .ok_or_else(|| js_error(&format!("missing calculator element `{selector}`")))?;

    element
        .dyn_into::<HtmlElement>()
        .map_err(|_| js_error(&format!("calculator element `{selector}` is not an HtmlElement")))
}

fn sync_view(
    state: &Rc<RefCell<CalculatorState>>,
    display: &HtmlElement,
    clear_button: &HtmlElement,
) {
    let state = state.borrow();
    display.set_text_content(Some(state.display()));
    clear_button.set_text_content(Some(state.clear_label()));
}

fn event_action(event: &Event) -> Option<Action> {
    let target = event.target()?;
    let element = target.dyn_into::<Element>().ok()?;
    let button = element.closest("[data-input]").ok().flatten()?;
    let token = button.get_attribute("data-input")?;
    action_from_token(&token)
}

fn js_error(message: &str) -> JsValue {
    js_sys::Error::new(message).into()
}

#[cfg(test)]
mod tests {
    use super::{action_from_key, display_name, state::Action, state::Operator, TOOL_NAME};

    #[test]
    fn exposes_tool_identity() {
        assert_eq!(TOOL_NAME, "calculator");
        assert_eq!(display_name(), "Calculator");
    }

    #[test]
    fn maps_keyboard_input() {
        assert_eq!(action_from_key("7"), Some(Action::Digit(7)));
        assert_eq!(action_from_key("+"), Some(Action::Operator(Operator::Add)));
        assert_eq!(action_from_key("Enter"), Some(Action::Equals));
    }
}
