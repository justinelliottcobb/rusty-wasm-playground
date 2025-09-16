// Dynamic import with hot-module replacement support
let wasm;

// Test GPU availability before loading WASM
function testGPUAvailability() {
  console.log('🔍 Testing GPU availability...');
  
  // Test WebGPU
  if (navigator.gpu) {
    console.log('✅ navigator.gpu exists');
    navigator.gpu.requestAdapter().then(adapter => {
      if (adapter) {
        console.log('✅ WebGPU adapter found:', adapter);
        console.log('  - Vendor:', adapter.info?.vendor || 'unknown');
        console.log('  - Architecture:', adapter.info?.architecture || 'unknown');
        console.log('  - Device:', adapter.info?.device || 'unknown');
      } else {
        console.log('❌ WebGPU adapter request returned null');
      }
    }).catch(err => {
      console.log('❌ WebGPU adapter request failed:', err);
    });
  } else {
    console.log('❌ navigator.gpu not available (WebGPU not supported)');
  }
  
  // Test WebGL
  const canvas = document.createElement('canvas');
  const gl = canvas.getContext('webgl2') || canvas.getContext('webgl');
  if (gl) {
    console.log('✅ WebGL context created successfully');
    console.log('  - Version:', gl.getParameter(gl.VERSION));
    console.log('  - Vendor:', gl.getParameter(gl.VENDOR));
    console.log('  - Renderer:', gl.getParameter(gl.RENDERER));
  } else {
    console.log('❌ WebGL context creation failed');
  }
}

async function loadWasm() {
  try {
    wasm = await import('./pkg/rusty_wasm_playground.js');
    await wasm.default();
    console.log('WASM module loaded successfully!');
    detectAvailableFeatures();
    setupEventListeners();
  } catch (err) {
    console.error('Failed to load WASM module:', err);
    document.getElementById('output').textContent = 'Failed to load WASM module. Check console for details.';
  }
}

function detectAvailableFeatures() {
  const features = {
    basic: true, // Always available
    random: typeof wasm.generate_random_data === 'function',
    math: typeof wasm.matrix_operations === 'function',
    gpu: typeof wasm.run_webgl_compute === 'function',
    sycamore: false // We'll detect this differently since it's on a separate page
  };

  console.log('📊 Detected WASM features:', features);

  // Show/hide sections based on available features
  Object.entries(features).forEach(([feature, available]) => {
    const elements = document.querySelectorAll(`[data-feature="${feature}"]`);
    elements.forEach(element => {
      if (available) {
        element.style.display = '';
      } else {
        element.style.display = 'none';
        console.log(`🔇 Hidden feature section: ${feature} (not available)`);
      }
    });
  });

  // Add feature indicator to page
  const featureStatus = Object.entries(features)
    .filter(([name]) => name !== 'basic') // Skip basic since it's always there
    .map(([name, available]) => `${name}: ${available ? '✅' : '❌'}`)
    .join(' | ');
  
  const statusDiv = document.createElement('div');
  statusDiv.style.cssText = 'position: fixed; bottom: 10px; right: 10px; background: rgba(0,0,0,0.8); color: white; padding: 8px; border-radius: 4px; font-size: 12px; z-index: 1000;';
  statusDiv.textContent = `Features: ${featureStatus}`;
  document.body.appendChild(statusDiv);
}

function setupEventListeners() {
  // Greet button
  document.getElementById('greet-btn').addEventListener('click', () => {
    const name = document.getElementById('name-input').value || 'World';
    const result = wasm.greet(name);
    document.getElementById('output').textContent = result;
  });

  // Fibonacci button
  document.getElementById('fib-btn').addEventListener('click', () => {
    const num = parseInt(document.getElementById('fib-input').value) || 10;
    const start = performance.now();
    const result = wasm.fibonacci(num);
    const end = performance.now();
    document.getElementById('output').textContent = 
      `Fibonacci(${num}) = ${result}\nComputation time: ${(end - start).toFixed(2)}ms`;
  });

  // DOM manipulation button
  document.getElementById('dom-btn').addEventListener('click', () => {
    wasm.set_text_content('output', `DOM updated from Rust at ${new Date().toLocaleTimeString()}`);
  });

  // Console log button
  document.getElementById('console-btn').addEventListener('click', () => {
    console.log('Button clicked - check for Rust message below:');
    // The main() function already logs when module loads
    document.getElementById('output').textContent = 'Check browser console for Rust messages';
  });

  // Render list button
  document.getElementById('render-list-btn').addEventListener('click', () => {
    const items = document.getElementById('list-input').value || 'Item 1, Item 2, Item 3';
    wasm.render_list('render-output', items);
  });

  // Render table button
  document.getElementById('render-table-btn').addEventListener('click', () => {
    wasm.render_table('render-output');
  });

  // WASM Crate Examples
  
  // Random data generation
  document.getElementById('random-data-btn').addEventListener('click', () => {
    const result = wasm.generate_random_data();
    document.getElementById('wasm-output').textContent = result;
  });

  // Password generation
  document.getElementById('password-btn').addEventListener('click', () => {
    const length = parseInt(document.getElementById('password-length').value) || 16;
    const password = wasm.generate_password(length);
    document.getElementById('wasm-output').textContent = `Generated password (${length} chars):\n${password}`;
  });

  // Dice rolling
  document.getElementById('dice-btn').addEventListener('click', () => {
    const numDice = parseInt(document.getElementById('num-dice').value) || 2;
    const sides = parseInt(document.getElementById('dice-sides').value) || 6;
    const result = wasm.roll_dice(numDice, sides);
    document.getElementById('wasm-output').textContent = result;
  });

  // Character generation with serde
  document.getElementById('character-btn').addEventListener('click', () => {
    const name = document.getElementById('character-name').value || 'Hero';
    const character = wasm.create_random_character(name);
    document.getElementById('wasm-output').textContent = 
      `Character: ${character.name}\n` +
      `Level: ${character.level}\n` +
      `Health: ${character.health}, Mana: ${character.mana}\n` +
      `Stats:\n` +
      `  Strength: ${character.stats.strength}\n` +
      `  Dexterity: ${character.stats.dexterity}\n` +
      `  Intelligence: ${character.stats.intelligence}\n` +
      `  Luck: ${character.stats.luck}`;
  });

  // Name generation
  document.getElementById('name-btn').addEventListener('click', () => {
    const name = wasm.generate_random_name();
    document.getElementById('wasm-output').textContent = `Random name: ${name}`;
  });

  // Card dealing
  document.getElementById('cards-btn').addEventListener('click', () => {
    const hand = wasm.shuffle_and_deal_cards();
    document.getElementById('wasm-output').textContent = hand;
  });

  // Linear Algebra Examples (using non-WASM-specific nalgebra crate)
  
  // Matrix analysis
  document.getElementById('matrix-btn').addEventListener('click', () => {
    const a11 = parseFloat(document.getElementById('m11').value) || 2;
    const a12 = parseFloat(document.getElementById('m12').value) || 1;
    const a21 = parseFloat(document.getElementById('m21').value) || 1;
    const a22 = parseFloat(document.getElementById('m22').value) || 3;
    const result = wasm.matrix_operations(a11, a12, a21, a22);
    document.getElementById('math-output').textContent = result;
  });

  // 3D transformations
  document.getElementById('transform-btn').addEventListener('click', () => {
    const x = parseFloat(document.getElementById('pt-x').value) || 1;
    const y = parseFloat(document.getElementById('pt-y').value) || 0;
    const z = parseFloat(document.getElementById('pt-z').value) || 0;
    const angle = parseFloat(document.getElementById('angle').value) || 45;
    const result = wasm.geometric_transformation(x, y, z, angle);
    document.getElementById('math-output').textContent = result;
  });

  // Vector operations
  document.getElementById('vector-btn').addEventListener('click', () => {
    const x1 = parseFloat(document.getElementById('v1-x').value) || 1;
    const y1 = parseFloat(document.getElementById('v1-y').value) || 0;
    const z1 = parseFloat(document.getElementById('v1-z').value) || 0;
    const x2 = parseFloat(document.getElementById('v2-x').value) || 0;
    const y2 = parseFloat(document.getElementById('v2-y').value) || 1;
    const z2 = parseFloat(document.getElementById('v2-z').value) || 0;
    const result = wasm.vector_operations(x1, y1, z1, x2, y2, z2);
    document.getElementById('math-output').textContent = result;
  });

  // Linear system solver
  document.getElementById('solve-btn').addEventListener('click', () => {
    const a11 = parseFloat(document.getElementById('a11').value) || 2;
    const a12 = parseFloat(document.getElementById('a12').value) || 1;
    const b1 = parseFloat(document.getElementById('b1').value) || 5;
    const a21 = parseFloat(document.getElementById('a21').value) || 1;
    const a22 = parseFloat(document.getElementById('a22').value) || 3;
    const b2 = parseFloat(document.getElementById('b2').value) || 7;
    const result = wasm.solve_linear_system(a11, a12, b1, a21, a22, b2);
    document.getElementById('math-output').textContent = result;
  });

  // WebGPU Examples (using non-WASM-specific wgpu crate)
  
  // GPU compute shader
  document.getElementById('compute-btn').addEventListener('click', async () => {
    try {
      const size = parseInt(document.getElementById('compute-size').value) || 1000;
      document.getElementById('gpu-output').textContent = 'Running GPU compute shader...';
      const result = await wasm.run_compute_shader(size);
      document.getElementById('gpu-output').textContent = result;
    } catch (error) {
      document.getElementById('gpu-output').textContent = `Error: ${error.message || error}\n\nThis may indicate:\n- WebGPU not supported in this browser\n- Graphics drivers need updating\n- Browser flags may need to be enabled`;
    }
  });

  // WebGL compute button (direct WebGL)
  document.getElementById('webgl-compute-btn').addEventListener('click', () => {
    try {
      const size = parseInt(document.getElementById('webgl-compute-size').value) || 1000;
      document.getElementById('gpu-output').textContent = 'Running WebGL compute...';
      const result = wasm.run_webgl_compute(size);
      document.getElementById('gpu-output').textContent = result;
    } catch (error) {
      document.getElementById('gpu-output').textContent = `Error: ${error.message || error}\n\nWebGL compute failed. This may indicate:\n- WebGL 2.0 not supported\n- Browser security restrictions\n- Graphics drivers need updating`;
    }
  });

  // GPU triangle rendering
  document.getElementById('triangle-btn').addEventListener('click', async () => {
    try {
      document.getElementById('gpu-output').textContent = 'Rendering GPU triangle...';
      const result = await wasm.render_triangle();
      document.getElementById('gpu-output').textContent = result;
    } catch (error) {
      document.getElementById('gpu-output').textContent = `Error: ${error.message || error}\n\nThis may indicate:\n- WebGPU not supported in this browser\n- Graphics drivers need updating\n- Browser flags may need to be enabled`;
    }
  });
}

// Test GPU availability first, then load WASM
testGPUAvailability();
loadWasm();

// Hot Module Replacement
if (import.meta.hot) {
  import.meta.hot.accept('./pkg/rusty_wasm_playground.js', async () => {
    console.log('WASM module updated, reloading...');
    await loadWasm();
  });
}