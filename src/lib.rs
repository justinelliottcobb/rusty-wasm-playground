use wasm_bindgen::prelude::*;
use web_sys::console;

mod examples;
#[cfg(feature = "sycamore")]
mod sycamore_app;

// Re-export all example functions for public API
pub use examples::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"Hello from Rust and WebAssembly!".into());
}