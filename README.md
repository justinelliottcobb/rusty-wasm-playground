# Rusty WASM Playground

A live-coding environment for Rust → WebAssembly development with hot-reloading!

## Quick Start

```bash
# Install required tools (if not already installed)
cargo install wasm-pack
cargo install cargo-watch

# Install dependencies
npm install

# Start development server with hot-reloading
npm run dev
```

Then open http://localhost:5173 in your browser.

## Features

- **Hot Module Replacement**: Changes to Rust code automatically rebuild and reload in the browser
- **Vite Dev Server**: Lightning-fast frontend development with instant updates
- **Interactive Examples**: Pre-built demos showing WASM capabilities
- **Developer Friendly**: Console error handling, performance timing, and clear error messages

## Project Structure

```
├── src/
│   └── lib.rs          # Your Rust/WASM code goes here
├── index.html          # Frontend HTML
├── main.js             # JavaScript glue code
├── pkg/                # Generated WASM bindings (auto-generated)
└── vite.config.js      # Vite configuration
```

## Development Workflow

1. Edit `src/lib.rs` - Your Rust code that compiles to WebAssembly
2. Save the file - cargo-watch automatically rebuilds the WASM module
3. Browser auto-reloads with your changes

## Available Functions

The template includes several example functions in `src/lib.rs`:

- `greet(name)` - Returns a greeting message
- `fibonacci(n)` - Calculates Fibonacci numbers
- `set_text_content(id, text)` - DOM manipulation from Rust

## Adding New Functions

1. Add your function to `src/lib.rs` with the `#[wasm_bindgen]` attribute:

```rust
#[wasm_bindgen]
pub fn my_function(input: &str) -> String {
    format!("Processing: {}", input)
}
```

2. Use it in `main.js`:

```javascript
const result = wasm.my_function("test");
```

## Build for Production

```bash
npm run build
```

This creates an optimized build in the `dist/` directory.

## Tips

- Use `web_sys` for DOM manipulation and browser APIs
- Check browser console for Rust panic messages
- The `console_error_panic_hook` is included for better error messages
- Fibonacci calculations above 40 might be slow (intentionally inefficient for demo)

## Troubleshooting

If you see "Failed to load WASM module":
1. Make sure `wasm-pack` is installed
2. Check that the initial build completed: `npm run wasm:build`
3. Look for error messages in the browser console

## Learn More

- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)