use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

mod sycamore_app;

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

// Example of rendering HTML elements from an array
#[wasm_bindgen]
pub fn render_list(container_id: &str, items: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Get the container element
    let container = document.get_element_by_id(container_id).unwrap();
    
    // Clear existing content
    container.set_inner_html("");
    
    // Create an unordered list
    let ul = document.create_element("ul").unwrap();
    
    // Split the items string (comma-separated) and create list items
    let items_vec: Vec<&str> = items.split(',').collect();
    
    for item in items_vec {
        let li = document.create_element("li").unwrap();
        li.set_text_content(Some(item.trim()));
        ul.append_child(&li).unwrap();
    }
    
    // Append the list to the container
    container.append_child(&ul).unwrap();
}

// More advanced example: render a table from structured data
#[wasm_bindgen]
pub fn render_table(container_id: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.get_element_by_id(container_id).unwrap();
    container.set_inner_html("");
    
    // Sample data - in a real app, this could be passed as parameters
    let languages = vec![
        ("Rust", "2015", "Systems Programming"),
        ("JavaScript", "1995", "Web Development"),
        ("Python", "1991", "General Purpose"),
        ("Go", "2009", "Cloud Infrastructure"),
    ];
    
    // Create table
    let table = document.create_element("table").unwrap();
    table.set_attribute("style", "border-collapse: collapse").unwrap();
    
    // Create header
    let thead = document.create_element("thead").unwrap();
    let header_row = document.create_element("tr").unwrap();
    
    for header in &["Language", "Year", "Primary Use"] {
        let th = document.create_element("th").unwrap();
        th.set_text_content(Some(header));
        th.set_attribute("style", "border: 1px solid #666; padding: 8px; background: #444").unwrap();
        header_row.append_child(&th).unwrap();
    }
    thead.append_child(&header_row).unwrap();
    table.append_child(&thead).unwrap();
    
    // Create body
    let tbody = document.create_element("tbody").unwrap();
    
    for (lang, year, use_case) in languages {
        let row = document.create_element("tr").unwrap();
        
        for cell_text in &[lang, year, use_case] {
            let td = document.create_element("td").unwrap();
            td.set_text_content(Some(cell_text));
            td.set_attribute("style", "border: 1px solid #666; padding: 8px").unwrap();
            row.append_child(&td).unwrap();
        }
        
        tbody.append_child(&row).unwrap();
    }
    
    table.append_child(&tbody).unwrap();
    container.append_child(&table).unwrap();
}

// WASM-optimized crate examples
// These demonstrate using crates specifically designed for WebAssembly

// Example 1: Random number generation with getrandom (WASM-optimized)
#[wasm_bindgen]
pub fn generate_random_data() -> String {
    let mut rng = thread_rng();
    
    let random_numbers: Vec<u32> = (0..10).map(|_| rng.gen_range(1..=100)).collect();
    let random_color = format!("#{:06x}", rng.gen::<u32>() & 0xFFFFFF);
    let random_float = rng.gen::<f64>();
    let random_bool = rng.gen::<bool>();
    
    format!(
        "Numbers: {:?}\nColor: {}\nFloat: {:.4}\nBoolean: {}",
        random_numbers, random_color, random_float, random_bool
    )
}

// Example 2: Password generator using rand crate
#[wasm_bindgen]
pub fn generate_password(length: usize) -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut rng = thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

// Example 3: Dice roller simulation
#[wasm_bindgen]
pub fn roll_dice(num_dice: u32, sides: u32) -> String {
    let mut rng = thread_rng();
    let mut rolls = Vec::new();
    let mut total = 0;
    
    for _ in 0..num_dice {
        let roll = rng.gen_range(1..=sides);
        rolls.push(roll);
        total += roll;
    }
    
    format!(
        "Rolled {}d{}: {:?}\nTotal: {} (Average: {:.1})",
        num_dice, sides, rolls, total, total as f64 / num_dice as f64
    )
}

// Example 4: Data structure with serde serialization (WASM-optimized)
#[derive(Serialize, Deserialize)]
struct GameCharacter {
    name: String,
    level: u32,
    health: u32,
    mana: u32,
    stats: CharacterStats,
}

#[derive(Serialize, Deserialize)]
struct CharacterStats {
    strength: u32,
    dexterity: u32,
    intelligence: u32,
    luck: u32,
}

#[wasm_bindgen]
pub fn create_random_character(name: &str) -> JsValue {
    let mut rng = thread_rng();
    
    let character = GameCharacter {
        name: name.to_string(),
        level: rng.gen_range(1..=50),
        health: rng.gen_range(50..=200),
        mana: rng.gen_range(30..=150),
        stats: CharacterStats {
            strength: rng.gen_range(10..=20),
            dexterity: rng.gen_range(10..=20),
            intelligence: rng.gen_range(10..=20),
            luck: rng.gen_range(5..=15),
        },
    };
    
    serde_wasm_bindgen::to_value(&character).unwrap()
}

// Example 5: Procedural name generator
const FIRST_NAMES: &[&str] = &[
    "Aiden", "Bella", "Connor", "Diana", "Ethan", "Fiona", "Gabriel", "Hannah",
    "Isaac", "Jade", "Kyle", "Luna", "Mason", "Nova", "Oscar", "Piper",
];

const LAST_NAMES: &[&str] = &[
    "Ashford", "Blake", "Cross", "Drake", "Evans", "Fox", "Gray", "Hunt",
    "Kane", "Lane", "Moore", "Nash", "Pierce", "Quinn", "Reed", "Stone",
];

#[wasm_bindgen]
pub fn generate_random_name() -> String {
    let mut rng = thread_rng();
    let first = FIRST_NAMES.choose(&mut rng).unwrap();
    let last = LAST_NAMES.choose(&mut rng).unwrap();
    format!("{} {}", first, last)
}

// Example 6: Shuffle and sample from collections
#[wasm_bindgen]
pub fn shuffle_and_deal_cards() -> String {
    let suits = ["♠", "♥", "♦", "♣"];
    let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    
    let mut deck = Vec::new();
    for suit in suits.iter() {
        for rank in ranks.iter() {
            deck.push(format!("{}{}", rank, suit));
        }
    }
    
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    
    let hand: Vec<String> = deck.iter().take(5).cloned().collect();
    
    format!("Your poker hand: {}", hand.join(", "))
}