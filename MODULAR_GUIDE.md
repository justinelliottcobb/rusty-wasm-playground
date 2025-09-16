# Modular Example System Guide

This document explains the modular example system implemented in the Rusty WASM Playground.

## Overview

The project has been refactored to use **feature flags** that allow easy enabling/disabling of different example categories. This makes it perfect as a bootstrap repository - you can easily remove examples you don't need and add your own.

## Feature Categories

### 🔧 `basic` (Always enabled)
- **Location**: `src/examples/basic.rs`
- **Dependencies**: None (core web-sys only)
- **Examples**: 
  - `greet()` - Simple string formatting
  - `fibonacci()` - Computation example
  - `set_text_content()` - DOM manipulation
  - `render_list()` - Dynamic HTML generation
  - `render_table()` - Complex HTML structures

### 🎲 `random` (Optional)
- **Location**: `src/examples/random.rs`
- **Dependencies**: `rand`, `getrandom`, `serde`, `serde-wasm-bindgen`
- **Examples**:
  - `generate_random_data()` - Random numbers, colors, booleans
  - `generate_password()` - Cryptographically secure passwords
  - `roll_dice()` - Dice simulation with statistics
  - `create_random_character()` - Complex data structures with serde
  - `generate_random_name()` - Procedural name generation
  - `shuffle_and_deal_cards()` - Collection algorithms

### 📐 `math` (Optional)
- **Location**: `src/examples/math.rs`
- **Dependencies**: `nalgebra`
- **Examples**:
  - `matrix_operations()` - Determinant, inverse, eigenvalues
  - `geometric_transformation()` - 3D rotations, scaling, translation
  - `vector_operations()` - Dot/cross products, projections
  - `solve_linear_system()` - Linear algebra with LU decomposition

### 🎮 `gpu` (Optional, experimental)
- **Location**: `src/examples/gpu.rs`
- **Dependencies**: `wgpu`, `wasm-bindgen-futures`, `futures-channel`, `bytemuck`
- **Examples**:
  - `run_webgl_compute()` - Direct WebGL verification
  - `run_compute_shader()` - WebGPU compute (when available)
  - `render_triangle()` - Graphics pipeline demo
- **Status**: ⚠️ Disabled by default due to WebGPU compatibility issues

### ⚛️ `sycamore` (Optional)
- **Location**: `src/sycamore_app.rs`
- **Dependencies**: `sycamore`
- **Examples**:
  - Reactive counter with state management
  - Dynamic todo list with fine-grained reactivity
  - Computed values and conditional rendering
- **Page**: Available at `/sycamore.html`

## Configuration Methods

### 1. Interactive Configuration (Recommended)

Use the provided JavaScript configuration system:

```javascript
// Load in Node.js or browser console
const { config } = require('./examples.config.js');

// View current status
config.printConfig();

// Toggle features
config.toggleFeature('gpu');        // Enable/disable GPU examples
config.toggleFeature('math');       // Enable/disable math examples

// Preset configurations
config.minimalBuild();              // Only basic examples
config.fullBuild();                 // All examples (including GPU)
config.disableProblematic();        // Disable known problematic features (GPU)
```

### 2. Manual Feature Control

Edit `Cargo.toml` directly:

```toml
# Minimal build (basic only)
[features]
default = ["basic"]

# Standard build (no GPU problems)
[features] 
default = ["basic", "random", "math", "sycamore"]

# Full build (all features, including experimental GPU)
[features]
default = ["basic", "random", "math", "gpu", "sycamore"]
```

### 3. Build Command Options

```bash
# Use default features (recommended for development)
npm run wasm:build

# Minimal build (basic only)
wasm-pack build --target web --no-default-features --features basic

# Custom feature combination
wasm-pack build --target web --no-default-features --features basic,random,math

# Build without problematic features
wasm-pack build --target web --no-default-features --features basic,random,math,sycamore

# Full build (including experimental GPU)
wasm-pack build --target web --no-default-features --features basic,random,math,gpu,sycamore
```

## Runtime Feature Detection

The JavaScript code automatically detects which features are available and:

1. **Hides unavailable sections** - HTML elements with `data-feature="feature_name"` are hidden if the feature isn't built
2. **Shows feature status** - A status indicator in the bottom-right shows which features are enabled
3. **Graceful degradation** - Missing functions are handled gracefully with error messages

Example HTML structure:
```html
<div class="demo-section" data-feature="math">
  <h2>Mathematical Computing</h2>
  <!-- This section only shows if math feature is enabled -->
</div>
```

## Bootstrap Usage

### Quick Start for New Projects

1. **Clone and customize**:
   ```bash
   git clone <this-repo> my-wasm-project
   cd my-wasm-project
   ```

2. **Remove unwanted examples**:
   ```bash
   # Remove GPU examples (recommended for most projects)
   rm src/examples/gpu.rs
   
   # Update src/examples/mod.rs to remove references
   # Edit Cargo.toml to remove dependencies
   ```

3. **Configure features**:
   ```bash
   # Edit Cargo.toml default features
   [features]
   default = ["basic", "random", "math"]  # Remove what you don't need
   ```

4. **Add your code**:
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

### Example Project Configurations

**Data Processing App**:
```toml
[features]
default = ["basic", "math"]  # Focus on computation
```

**Interactive UI Demo**:
```toml
[features]
default = ["basic", "random", "sycamore"]  # UI + dynamic content
```

**Game/Simulation**:
```toml
[features]
default = ["basic", "random", "math"]  # Physics + randomness
```

**Minimal Library**:
```toml
[features]
default = ["basic"]  # Just core functionality
```

## Development Workflow

1. **Configure features** for your use case
2. **Remove unwanted examples** to reduce bundle size
3. **Add your custom modules** following the existing pattern
4. **Update HTML/JS** to match your feature set
5. **Test different builds** to ensure modularity works
6. **Deploy** your specialized WASM application

## Benefits

- ✅ **Faster builds** - Only compile what you need
- ✅ **Smaller bundles** - Fewer dependencies in final WASM
- ✅ **Easy maintenance** - Clear separation of concerns
- ✅ **Bootstrap friendly** - Remove examples, add your code
- ✅ **Future-proof** - Easily disable problematic features
- ✅ **Learning tool** - Enable/disable to understand dependencies

## Troubleshooting

### Build Issues
```bash
# Clean rebuild fixes most issues
cargo clean && rm -rf pkg/ target/
npm run wasm:build
```

### Feature Detection Issues
- Check browser console for WASM loading errors
- Verify feature functions exist in compiled module
- Check HTML `data-feature` attributes match Cargo features

### GPU Features
- GPU examples disabled by default due to browser compatibility
- Enable only for testing: `--features gpu`
- Requires specific browser flags and hardware support

---

This modular system makes the Rusty WASM Playground perfect as a starting point for any WebAssembly project. Clone, configure, customize, and deploy! 🚀