use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! From Rust/WASM", name)
}

#[wasm_bindgen]
pub fn set_text_content(id: &str, text: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id(id).unwrap();
    element.set_text_content(Some(text));
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen]
pub fn render_list(container_id: &str, items: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.get_element_by_id(container_id).unwrap();
    container.set_inner_html("");
    
    let ul = document.create_element("ul").unwrap();
    let items_vec: Vec<&str> = items.split(',').collect();
    
    for item in items_vec {
        let li = document.create_element("li").unwrap();
        li.set_text_content(Some(item.trim()));
        ul.append_child(&li).unwrap();
    }
    
    container.append_child(&ul).unwrap();
}

#[wasm_bindgen]
pub fn render_table(container_id: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.get_element_by_id(container_id).unwrap();
    container.set_inner_html("");
    
    let languages = vec![
        ("Rust", "2015", "Systems Programming"),
        ("JavaScript", "1995", "Web Development"),
        ("Python", "1991", "General Purpose"),
        ("Go", "2009", "Cloud Infrastructure"),
    ];
    
    let table = document.create_element("table").unwrap();
    table.set_attribute("style", "border-collapse: collapse").unwrap();
    
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