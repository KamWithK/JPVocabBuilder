mod utils;
mod scoring;

use std::ffi::OsStr;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

use subparse::{get_subtitle_format, parse_str};
use tinysegmenter::tokenize;

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
pub fn parse_subtitle(raw_content: &str, extention: &str) {
    let sub_format = get_subtitle_format(Some(OsStr::new(extention)), raw_content.as_bytes()).expect("Unknown Subtitle Format");
    let subtitle = parse_str(sub_format, raw_content, 25.0).expect("Subtitle Parsing Error");
    let subtitle_entries = subtitle.get_subtitle_entries().expect("Unexpected Error Reading Subtitles");

    for subtitle_entry in subtitle_entries {
        if let Some(sub_line) = subtitle_entry.line {
            tokenize(&sub_line).iter().for_each(|word| SCOREBOARD.lock().unwrap().request_add(word.to_string()));
        }
    }
}

#[wasm_bindgen]
pub fn get_best() {
    SCOREBOARD.lock().unwrap().top_words().iter().for_each(|item| println!("{}", item.word));
}
