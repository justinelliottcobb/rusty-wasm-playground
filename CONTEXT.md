# Project Context

## Overview
Rusty WASM Playground is a live-coding environment for Rust → WebAssembly development with hot-reloading capabilities similar to modern JavaScript frameworks.

## Architecture

### Core Setup
- **Build Tool**: wasm-pack for compiling Rust to WebAssembly
- **Dev Server**: Vite with hot-module replacement
- **Auto-rebuild**: cargo-watch monitors Rust files and triggers rebuilds
- **Target**: WebAssembly with wasm-bindgen for JS interop

### Project Structure
```
├── src/
│   ├── lib.rs           # Main WASM module with DOM manipulation examples
│   └── sycamore_app.rs  # Sycamore reactive framework examples
├── pkg/                  # Generated WASM bindings (auto-generated)
├── index.html           # Main demo page with vanilla WASM examples
├── sycamore.html        # Sycamore framework examples page
├── main.js              # JavaScript glue for vanilla examples
└── vite.config.js       # Dev server configuration
```

## Features

### Vanilla WASM Examples (`src/lib.rs`)
- Direct DOM manipulation from Rust
- Fibonacci calculation demonstrating computation
- Dynamic HTML generation (lists and tables)
- Console logging integration

### Sycamore Framework Examples (`src/sycamore_app.rs`)
- **Counter**: Basic signals and reactive state
- **Todo List**: Dynamic lists with two-way binding
- **Reactive Form**: Computed values and conditional rendering
- Demonstrates fine-grained reactivity without virtual DOM

## Development Workflow

### Quick Start
```bash
npm run dev  # Starts on port 3000
```

### What Happens
1. `wasm-pack build` compiles Rust to WASM
2. `cargo-watch` monitors for Rust file changes
3. Vite serves the frontend with HMR
4. Changes to Rust code trigger automatic rebuild and browser refresh

### Configuration
- **Port**: 3000 (configured for remote access)
- **Allowed Hosts**: Configured via `.env` file (not committed)
- **Build Mode**: Development mode for fast iteration

## Key Technologies

### Rust Dependencies
- `wasm-bindgen`: Rust/JS interop
- `web-sys`: Browser API bindings
- `sycamore`: Reactive UI framework
- `console_error_panic_hook`: Better error messages

### JavaScript Dependencies
- `vite`: Dev server and bundler
- `concurrently`: Run multiple processes

## Environment Variables
- `VITE_ALLOWED_HOSTS`: Comma-separated list of allowed hostnames
- Configured in `.env` (gitignored) based on `.env.example`

## Learning Path

### For Beginners
1. Start with `index.html` - see basic WASM interactions
2. Modify `greet()` function in `src/lib.rs`
3. Try changing the fibonacci implementation
4. Experiment with DOM manipulation functions

### For Framework Users
1. Navigate to `/sycamore.html`
2. Explore reactive patterns in `src/sycamore_app.rs`
3. Modify the Todo component to add new features
4. Create new reactive components

## Performance Notes
- Development builds are unoptimized for faster compilation
- Production builds use `npm run build` for optimization
- Fibonacci example intentionally inefficient for demo purposes
- Sycamore uses fine-grained reactivity (no virtual DOM)

## Troubleshooting

### WASM Module Failed to Load
- Ensure `wasm-pack` is installed: `cargo install wasm-pack`
- Check browser console for detailed errors
- Verify `pkg/` directory exists after build

### Hot Reload Not Working
- Check that `cargo-watch` is installed: `cargo install cargo-watch`
- Ensure file watchers aren't hitting OS limits
- Restart dev server if needed

### Port 3000 Already in Use
- The config uses `strictPort: true` - will fail if occupied
- Stop other services on port 3000 or modify `vite.config.js`

## Future Enhancements
- Add more Sycamore examples (routing, async data)
- Integrate other WASM frameworks (Yew, Leptos)
- Add WebGL/Canvas examples
- Include WASM-based data processing demos
- Add performance benchmarking tools