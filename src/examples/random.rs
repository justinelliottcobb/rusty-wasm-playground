#[cfg(feature = "random")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "random")]
use rand::prelude::*;
#[cfg(feature = "random")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "random")]
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

#[cfg(feature = "random")]
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

#[cfg(feature = "random")]
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

#[cfg(feature = "random")]
#[derive(Serialize, Deserialize)]
struct GameCharacter {
    name: String,
    level: u32,
    health: u32,
    mana: u32,
    stats: CharacterStats,
}

#[cfg(feature = "random")]
#[derive(Serialize, Deserialize)]
struct CharacterStats {
    strength: u32,
    dexterity: u32,
    intelligence: u32,
    luck: u32,
}

#[cfg(feature = "random")]
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

#[cfg(feature = "random")]
const FIRST_NAMES: &[&str] = &[
    "Aiden", "Bella", "Connor", "Diana", "Ethan", "Fiona", "Gabriel", "Hannah",
    "Isaac", "Jade", "Kyle", "Luna", "Mason", "Nova", "Oscar", "Piper",
];

#[cfg(feature = "random")]
const LAST_NAMES: &[&str] = &[
    "Ashford", "Blake", "Cross", "Drake", "Evans", "Fox", "Gray", "Hunt",
    "Kane", "Lane", "Moore", "Nash", "Pierce", "Quinn", "Reed", "Stone",
];

#[cfg(feature = "random")]
#[wasm_bindgen]
pub fn generate_random_name() -> String {
    let mut rng = thread_rng();
    let first = FIRST_NAMES.choose(&mut rng).unwrap();
    let last = LAST_NAMES.choose(&mut rng).unwrap();
    format!("{} {}", first, last)
}

#[cfg(feature = "random")]
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