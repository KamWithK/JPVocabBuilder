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
    pub fn request_add(&mut self, word: String) {
        // Add new words and update existing ones frequency
        if self.words.iter_mut().all(|item| {
            let found = item.word == word;
            if found {
                item.frequency += 1;
            }

            !found
        }) {
            self.words.push(Word::new(word, 1));
        }
    }

    pub fn top_words(&self) -> Vec<&Word> {
        let (mut min_frequency, mut min_index) = (usize::MAX, 0);
        let mut top_words: Vec<&Word> = Vec::default();

        for item in &self.words {
            if top_words.len() < self.limit {
                top_words.push(item);

                if item.frequency < min_frequency {
                    min_frequency = item.frequency;
                    min_index = top_words.len() - 1;
                }
            } else if item.frequency > min_frequency {
                let _ = std::mem::replace(&mut top_words[min_index], item);
                min_frequency = item.frequency;
            }
        }

        top_words
    }
}