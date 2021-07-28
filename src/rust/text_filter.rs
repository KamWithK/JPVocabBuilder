// Modified version of: https://github.com/PSeitz/wana_kana_rust
use crate::char_ranges::{JAPANESE_RANGES, KANJI};

pub fn in_range(char: char, start: u32, end: u32) -> bool {
    start <= char as u32 && char as u32 <= end
}

pub fn is_char_japanese(character: char) -> bool {
    JAPANESE_RANGES.iter().any(|el: &[u32; 2]| in_range(character, el[0], el[1]))
}

pub fn is_japanese(input: &str) -> String {
    input.chars().filter(|character| is_char_japanese(*character)).collect()
}

pub fn relevant(word: &str) -> bool {
    word.len() > 3 || word.chars().any(|character| in_range(character, KANJI[0], KANJI[1]))
}
