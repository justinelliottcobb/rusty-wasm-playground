use wasm_bindgen::prelude::*;
use web_sys::console;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"Hello from Rust and WebAssembly!".into());
}

// Export a `greet` function from Rust to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! From Rust/WASM", name)
}

// Example of manipulating the DOM
#[wasm_bindgen]
pub fn set_text_content(id: &str, text: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(id).unwrap();
    element.set_text_content(Some(text));
}

// Example of a computation-heavy function
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}