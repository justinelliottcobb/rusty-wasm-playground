// Dynamic import with hot-module replacement support
let wasm;

async function loadWasm() {
  try {
    wasm = await import('./pkg/rusty_wasm_playground.js');
    await wasm.default();
    console.log('WASM module loaded successfully!');
    setupEventListeners();
  } catch (err) {
    console.error('Failed to load WASM module:', err);
    document.getElementById('output').textContent = 'Failed to load WASM module. Check console for details.';
  }
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
}

// Initial load
loadWasm();

// Hot Module Replacement
if (import.meta.hot) {
  import.meta.hot.accept('./pkg/rusty_wasm_playground.js', async () => {
    console.log('WASM module updated, reloading...');
    await loadWasm();
  });
}