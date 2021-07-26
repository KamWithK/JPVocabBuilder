mod utils;
mod scoring;
mod text_filter;

#[allow(dead_code)]
mod char_ranges;

use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

use tinysegmenter::tokenize;
use text_filter::keep_japanese;

use wasm_bindgen::prelude::wasm_bindgen;

use scoring::Scoreboard;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref SCOREBOARD: Arc<Mutex<Scoreboard>> = Arc::new(Mutex::new(Scoreboard {
        limit: 100,
        words: Vec::default()
    }));
}

#[wasm_bindgen]
extern {
}

#[wasm_bindgen]
pub fn parse_subtitle(content: &str) {
    tokenize(&keep_japanese(content)).iter().for_each(|word| SCOREBOARD.lock().unwrap().request_add(word.to_string()));
}

#[wasm_bindgen]
pub fn get_best() -> String {
    let strings: Vec<String> = SCOREBOARD.lock().unwrap().top_words().iter().map(|item| format!("{{\"word\":\"{}\", \"frequency\":{}}}", &item.word, &item.frequency)).collect();
    "[".to_string() + &strings.join(",") + "]"
}
