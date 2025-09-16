# Rusty WASM Playground

A modular Rust → WebAssembly development environment with hot-reloading and configurable example sets. Perfect as a bootstrap repository for rapid WASM project development.

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

## 🧩 Modular Example System

This project features **modular examples** that can be easily enabled, disabled, or removed. Each feature set is isolated with its own dependencies and can be toggled via Cargo features.

### Available Feature Sets

| Feature | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| `basic` | ✅ Always enabled | Core WASM examples (greet, fibonacci, DOM) | None |
| `random` | ✅ Enabled by default | Random data, passwords, dice, character creation | `rand`, `getrandom`, `serde` |
| `math` | ✅ Enabled by default | Linear algebra, matrix ops, 3D transforms | `nalgebra` |
| `gpu` | ❌ Disabled by default | WebGPU/WebGL graphics and compute | `wgpu`, `wasm-bindgen-futures` |
| `sycamore` | ✅ Enabled by default | Reactive framework examples | `sycamore` |

### Configuration Management

#### Interactive Configuration

Load the configuration script:

```javascript
// In Node.js or browser console
const { config } = require('./examples.config.js');

// View current configuration
config.printConfig();

// Toggle specific features
config.toggleFeature('gpu');        // Enable/disable GPU examples
config.toggleFeature('math');       // Enable/disable math examples

// Preset configurations
config.minimalBuild();              // Only basic examples
config.fullBuild();                 // All examples (including problematic GPU)
config.disableProblematic();        // Disable known problematic features
```

#### Manual Feature Control

Edit `Cargo.toml` directly:

```toml
# Minimal build (basic only)
[features]
default = ["basic"]

# Standard build (no GPU)
[features] 
default = ["basic", "random", "math", "sycamore"]

# Full build (all features)
[features]
default = ["basic", "random", "math", "gpu", "sycamore"]
```

### Build Commands

```bash
# Build with default features
npm run wasm:build

# Build with specific features
wasm-pack build --target web --features basic,random,math

# Build minimal (basic only)
wasm-pack build --target web --features basic

# Build without problematic GPU features  
wasm-pack build --target web --features basic,random,math,sycamore
```

## 🎨 Customizing for Your Project

### 1. Remove Unwanted Examples

```bash
# Remove a specific example module
rm src/examples/gpu.rs

# Update src/examples/mod.rs to remove the module reference
# Update Cargo.toml to remove the feature and dependencies
```

### 2. Add New Example Categories

```rust
// Create src/examples/my_feature.rs
#[cfg(feature = "my_feature")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "my_feature")]
#[wasm_bindgen]
pub fn my_function() -> String {
    "Hello from my custom feature!".to_string()
}
```

```toml
# Add to Cargo.toml
[features]
my_feature = ["dep:my_crate"]

[dependencies]
my_crate = { version = "1.0", optional = true }
```

## 🚀 Bootstrap Usage

This repository is designed to be cloned and customized:

1. **Fork/clone** this repository
2. **Remove unwanted examples** using the configuration system
3. **Add your specific functionality** following the modular pattern
4. **Customize the HTML/CSS** for your use case
5. **Deploy** your specialized WASM application

Perfect for:
- Rapid WASM prototyping
- Learning Rust/WASM development
- Building specialized WASM applications
- Creating WASM demos and tutorials

## Learn More

- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)