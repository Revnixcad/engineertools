pub fn float_to_clipboard(float_value: f64) -> impl Fn(web_sys::MouseEvent) {
    let copy_to_clipboard = move |_| {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator(); // Access the Navigator object
            let clipboard = navigator.clipboard();
            let result_text = format!("{:.2}", float_value);
            let _ = clipboard.write_text(&result_text);
            leptos::logging::log!("Copied to clipboard: {}", result_text);
        } else {
            leptos::logging::error!("Window object not available");
        }
    };
    copy_to_clipboard
}
