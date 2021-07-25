mod scoring;

use std::path::{PathBuf};
use std::io::Read;
use std::fs::{File, read_dir};

use subparse::{get_subtitle_format, parse_str};
use tinysegmenter::tokenize;

use scoring::Scoreboard;

fn parse_subtitle(path: PathBuf, scoreboard: &mut Scoreboard) {
    let mut file_content = String::default();
    File::open(&path).unwrap().read_to_string(&mut file_content).unwrap();

    let sub_format = get_subtitle_format(path.extension(), file_content.as_bytes()).expect("Unknown Subtitle Format");
    let subtitle = parse_str(sub_format, &file_content, 25.0).expect("Subtitle Parsing Error");
    let subtitle_entries = subtitle.get_subtitle_entries().expect("Unexpected Error Reading Subtitles");

    for subtitle_entry in subtitle_entries {
        if let Some(sub_line) = subtitle_entry.line {
            tokenize(&sub_line).iter().for_each(|word| scoreboard.request_add(word.to_string()));
        }
    }
}

fn main() {
    let paths = read_dir("C:/Users/kamwi/Downloads/subs").unwrap();

    let mut scoreboard = Scoreboard {
        limit: 20,
        words: Vec::default()
    };

    paths.for_each(|path| parse_subtitle(path.unwrap().path(), &mut scoreboard));
    scoreboard.top_words().iter().for_each(|item| println!("{}", item.word));
}
