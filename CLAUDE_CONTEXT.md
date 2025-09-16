# Rusty WASM Playground - Project Context

## Project Overview
A **modular** live-coding environment for Rust → WebAssembly development with hot-reloading and configurable example sets. Perfect as a bootstrap repository for rapid WASM project development. Features include GPU acceleration examples, framework integration, and mathematical computing demonstrations - all independently toggleable via Cargo feature flags.

## Key Features Implemented

### 1. Live Development Environment ✅
- **Hot-reloading**: Vite dev server with HMR + cargo-watch for automatic Rust rebuilding
- **Port Configuration**: Runs on port 3000 with remote access via environment variables
- **Build System**: wasm-pack + concurrent builds for seamless development

### 2. Framework Integration ✅
- **Sycamore Framework**: Reactive examples with fine-grained reactivity
  - Counter component with state management
  - Todo list with dynamic updates
  - Reactive forms and data binding
- **HTML Rendering**: Functions to render lists and tables from Rust arrays

### 3. Crate Examples ✅
- **WASM-Specific Crates**: rand, getrandom, serde-wasm-bindgen
  - Random data generation, password generation, dice rolling
  - Character creation with JSON serialization
  - Name generation and card shuffling
- **Non-WASM-Specific Crates**: nalgebra linear algebra library
  - Matrix operations (determinant, inverse, eigenvalues)
  - 3D geometric transformations (rotations, scaling, translation)
  - Vector operations (dot/cross products, projections)
  - Linear system solving with LU decomposition

### 4. GPU Computing Attempts
- **Target**: wgpu + WGSL for GPU compute shaders
- **Challenge**: WebGPU JavaScript API not available despite hardware support
- **Browser Status**: 
  - chrome://gpu shows WebGPU hardware accelerated with multiple adapters
  - navigator.gpu undefined (JavaScript API not exposed)
  - WebGL 2.0 working perfectly
- **wgpu Issues**: Adapter detection timeouts on both WebGPU and WebGL backends
- **Solution**: Direct WebGL demo using web-sys for GPU verification

### 5. Modular Architecture ✅ **NEW**
- **Feature-Based Organization**: Each example category in separate modules with feature flags
- **Optional Dependencies**: Only compile what you need via Cargo features
- **Runtime Detection**: JavaScript automatically detects available features and adapts UI
- **Bootstrap-Friendly**: Easy to remove unwanted examples, perfect for starting new projects

### 6. Files Structure
```
├── Cargo.toml              # Feature flags + optional dependencies
├── package.json            # npm scripts for build/dev workflow  
├── vite.config.js          # Vite config with port 3000 + remote access
├── index.html              # Main demo page with feature detection
├── sycamore.html           # Sycamore framework examples
├── main.js                 # Feature detection + WASM loading
├── examples.config.js      # Interactive configuration management
├── MODULAR_GUIDE.md        # Detailed modular system documentation
├── src/
│   ├── lib.rs              # Main entry point (minimal)
│   ├── examples/           # Modular example organization
│   │   ├── mod.rs          # Feature-gated exports
│   │   ├── basic.rs        # Core WASM (always enabled)
│   │   ├── random.rs       # Random data (#[cfg(feature = "random")])
│   │   ├── math.rs         # Linear algebra (#[cfg(feature = "math")])
│   │   └── gpu.rs          # Graphics/compute (#[cfg(feature = "gpu")])
│   ├── sycamore_app.rs     # Reactive framework (#[cfg(feature = "sycamore")])
│   ├── compute.wgsl        # WGSL compute shader (preserved but disabled)
│   └── triangle.wgsl       # WGSL vertex/fragment shaders (preserved but disabled)
```

## Current Status

### ✅ Working Features
1. **Live-coding environment** with instant hot-reload
2. **Modular feature system** with runtime detection and configuration
3. **All basic Rust/WASM examples** (greet, fibonacci, DOM manipulation)
4. **HTML rendering from arrays** (dynamic lists and tables) 
5. **Sycamore reactive framework** with working components (feature-gated)
6. **Random data generation** using WASM-optimized crates (feature-gated)
7. **Advanced mathematics** using nalgebra (matrix ops, 3D transforms, etc.) (feature-gated)
8. **WebGL GPU verification** - proves GPU accessible from Rust WASM (feature-gated)
9. **Feature detection UI** - automatically shows/hides sections based on build
10. **Bootstrap-ready structure** - easy to clone, customize, and deploy

### ⚠️ Experimental (Disabled by Default)
- **wgpu GPU compute**: Preserved in `gpu` feature but disabled due to WebGPU compatibility
- **Complex WebGL compute**: Basic GPU operations work, full compute shaders need more setup

### ✅ Resolved Issues
- **WebGPU compatibility problems**: Now isolated in optional `gpu` feature
- **Monolithic structure**: Refactored into modular, feature-based architecture
- **Bootstrap unfriendly**: Now easily customizable by removing unwanted features

## Technical Insights Learned

### wgpu vs WebGPU Clarification
- **WebGPU** = Browser API specification (like WebGL)
- **wgpu** = Cross-platform Rust graphics library targeting multiple backends:
  - Native: Vulkan, Metal, DirectX 12
  - Web: WebGPU (when available) OR WebGL (fallback)
- **The Issue**: wgpu's browser integration has compatibility gaps

### WebGPU Browser Support Challenge
- Hardware support exists (confirmed via chrome://gpu)
- JavaScript API not exposed to web pages
- Requires specific chrome://flags configuration that varies by browser version
- Even with flags enabled, navigator.gpu may remain undefined

### Build System Lessons
- **wasm-bindgen version consistency** critical for avoiding binding errors
- **cargo clean + rebuild** often required when adding/removing dependencies
- **web-sys feature flags** must match actual API usage
- **js-sys dependency** needed for JavaScript array/object manipulation

## Development Commands

```bash
# Start development server (default features)
npm run dev

# Build WASM manually  
npm run wasm:build

# Modular builds
wasm-pack build --target web --no-default-features --features basic                    # Minimal
wasm-pack build --target web --features basic,random,math,sycamore                     # Standard 
wasm-pack build --target web --features basic,random,math,gpu,sycamore                 # Full + GPU

# Configuration management
node -e "require('./examples.config.js').config.printConfig()"                        # View features
node -e "require('./examples.config.js').config.disableProblematic()"                 # Disable GPU

# Clean rebuild (when binding errors occur)
cargo clean && rm -rf pkg/ target/
npm run wasm:build

# Environment variables for remote access
export VITE_ALLOWED_HOSTS="your-hostname"
npm run dev
```

## Browser Setup for WebGPU Testing

Enable these chrome://flags (may vary by Chrome version):
- `#enable-unsafe-webgpu`
- `#enable-webgpu-developer-features` 
- `#enable-experimental-web-platform-features`
- `#force-high-performance-gpu`

## Next Steps / TODO

1. **Bootstrap Enhancements**:
   - Create project templates for common use cases (data processing, games, UI apps)
   - Add CI/CD configuration examples
   - Create deployment guides for different platforms

2. **Enhanced WebGL Compute**:
   - Implement transform feedback for real GPU compute
   - Add more sophisticated WebGL shader examples
   - Create WebGL-based parallel processing demos

3. **Framework Expansion**:
   - Add more Sycamore components (routing, state management)
   - Integrate other Rust web frameworks (Yew, Leptos) as optional features
   - Create more complex UI examples

4. **Mathematical Computing**:
   - Add scientific computing examples (differential equations, FFT)
   - Integrate plotting/visualization libraries
   - Create interactive mathematical demonstrations

5. **WebGPU Future-Proofing**:
   - Monitor Chrome WebGPU API availability updates
   - Test different chrome://flags combinations when support improves
   - Consider WebGPU polyfills or alternatives

## Key Dependencies (Now Modular)

```toml
# Core dependencies (always included)
[dependencies]
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"
web-sys = { version = "0.3", features = ["WebGl2RenderingContext", "Performance", ...] }

# Optional feature-based dependencies
sycamore = { version = "0.8", optional = true }           # Reactive framework
rand = { version = "0.8", optional = true }               # Random number generation
getrandom = { version = "0.2", optional = true }          # WASM-optimized entropy
serde = { version = "1.0", optional = true }              # Serialization
nalgebra = { version = "0.33", optional = true }          # Linear algebra
wgpu = { version = "0.19", optional = true }              # Graphics/compute (experimental)

[features]
default = ["basic", "random", "math", "sycamore"]         # Stable features only
basic = []                                                # Core WASM examples
random = ["dep:rand", "dep:getrandom", "dep:serde"]       # Random data generation
math = ["dep:nalgebra"]                                   # Mathematical computing
gpu = ["dep:wgpu", "dep:wasm-bindgen-futures"]            # Graphics (disabled by default)
sycamore = ["dep:sycamore"]                               # Reactive framework
```

## Project Impact

This project successfully demonstrates:

✅ **Modular Rust/WASM Development**: Feature-based architecture enables selective compilation  
✅ **Bootstrap-Ready Structure**: Perfect starting point for new WASM projects  
✅ **Complex Library Integration**: Both WASM-specific and general Rust crates work seamlessly  
✅ **Live Development Workflow**: Hot-reloading with instant feedback  
✅ **Future-Proof Design**: Experimental features isolated and easy to enable when ready  
✅ **Production-Ready Patterns**: Clean separation, feature detection, graceful degradation

**Perfect for**: Learning Rust/WASM, rapid prototyping, production applications, and serving as a foundation for specialized WASM projects.