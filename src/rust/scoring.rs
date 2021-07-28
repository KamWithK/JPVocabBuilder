use crate::text_filter::relevant;

pub struct Word {
    pub word: String,
    pub frequency: usize
}

impl Word {
    pub fn new(word: String, frequency: usize) -> Self {
        Word {
            word,
            frequency
        }
    }
}

pub struct Scoreboard {
    pub limit: usize,
    pub words: Vec<Word>
}

impl Scoreboard {
    pub fn request_add(&mut self, word: &str) {
        // Add new words and update existing ones frequency
        if relevant(word) && self.words.iter_mut().all(|item| {
            let found = item.word == word;
            if found {
                item.frequency += 1;
            }

            !found
        }) {
            self.words.push(Word::new(word.to_string(), 1));
        }
    }

    pub fn top_words(&mut self) -> &[Word] {
        self.words.sort_by(|word_one, word_two| word_two.frequency.cmp(&word_one.frequency));
        &self.words[..self.limit]
    }
}
