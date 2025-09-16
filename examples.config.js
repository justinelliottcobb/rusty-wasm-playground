// Example Configuration System
// This file allows you to easily enable/disable different feature sets

const EXAMPLE_FEATURES = {
  // Basic examples - always enabled (no dependencies)
  basic: {
    enabled: true,
    required: true,
    description: "Basic WASM examples (greet, fibonacci, DOM manipulation)",
    dependencies: []
  },

  // Random number generation and data structures
  random: {
    enabled: true,
    description: "Random data generation, password creation, dice rolling, character creation",
    dependencies: ["rand", "getrandom", "serde", "serde-wasm-bindgen"]
  },

  // Mathematical computing with nalgebra
  math: {
    enabled: true,
    description: "Linear algebra, matrix operations, 3D transformations, vector math",
    dependencies: ["nalgebra"]
  },

  // GPU computing (problematic - disabled by default)
  gpu: {
    enabled: false,
    description: "WebGPU/WebGL graphics and compute shaders (experimental/problematic)",
    dependencies: ["wgpu", "wasm-bindgen-futures", "futures-channel", "bytemuck"],
    notes: "Currently has WebGPU compatibility issues. Enable only for testing."
  },

  // Sycamore reactive framework
  sycamore: {
    enabled: true,
    description: "Reactive web framework examples with fine-grained reactivity",
    dependencies: ["sycamore"]
  }
};

// Generate cargo features string
function generateCargoFeatures() {
  const enabledFeatures = Object.entries(EXAMPLE_FEATURES)
    .filter(([name, config]) => config.enabled)
    .map(([name]) => name);
  
  return enabledFeatures.join(',');
}

// Generate build command
function generateBuildCommand() {
  const features = generateCargoFeatures();
  return `wasm-pack build --target web --features ${features}`;
}

// Configuration management functions
const config = {
  // Get current enabled features
  getEnabledFeatures() {
    return Object.entries(EXAMPLE_FEATURES)
      .filter(([name, config]) => config.enabled)
      .reduce((acc, [name, config]) => {
        acc[name] = config;
        return acc;
      }, {});
  },

  // Toggle a feature
  toggleFeature(featureName) {
    if (EXAMPLE_FEATURES[featureName] && !EXAMPLE_FEATURES[featureName].required) {
      EXAMPLE_FEATURES[featureName].enabled = !EXAMPLE_FEATURES[featureName].enabled;
      console.log(`${featureName}: ${EXAMPLE_FEATURES[featureName].enabled ? 'enabled' : 'disabled'}`);
    } else if (EXAMPLE_FEATURES[featureName]?.required) {
      console.log(`${featureName} is required and cannot be disabled`);
    } else {
      console.log(`Unknown feature: ${featureName}`);
    }
  },

  // Disable problematic features (like GPU)
  disableProblematic() {
    EXAMPLE_FEATURES.gpu.enabled = false;
    console.log('Disabled problematic GPU features');
  },

  // Enable only basic features
  minimalBuild() {
    Object.keys(EXAMPLE_FEATURES).forEach(name => {
      if (!EXAMPLE_FEATURES[name].required) {
        EXAMPLE_FEATURES[name].enabled = false;
      }
    });
    console.log('Enabled minimal build (basic features only)');
  },

  // Enable all features
  fullBuild() {
    Object.keys(EXAMPLE_FEATURES).forEach(name => {
      EXAMPLE_FEATURES[name].enabled = true;
    });
    console.log('Enabled full build (all features)');
  },

  // Print current configuration
  printConfig() {
    console.log('\nCurrent Feature Configuration:');
    console.log('==============================');
    Object.entries(EXAMPLE_FEATURES).forEach(([name, config]) => {
      const status = config.enabled ? '✅ ENABLED' : '❌ disabled';
      const required = config.required ? ' (required)' : '';
      console.log(`${name}: ${status}${required}`);
      console.log(`  ${config.description}`);
      if (config.dependencies.length > 0) {
        console.log(`  Dependencies: ${config.dependencies.join(', ')}`);
      }
      if (config.notes) {
        console.log(`  Notes: ${config.notes}`);
      }
      console.log('');
    });
    
    console.log(`Cargo features: --features ${generateCargoFeatures()}`);
    console.log(`Build command: ${generateBuildCommand()}`);
  }
};

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    EXAMPLE_FEATURES,
    generateCargoFeatures,
    generateBuildCommand,
    config
  };
}

// Browser/global usage
if (typeof window !== 'undefined') {
  window.ExampleConfig = config;
}