use alloc::format;
use alloc::string::{String, ToString};
use core::str::Chars;
use serde_json::{Map, Value};

pub struct BudouxParser<'a> {
    model: &'a Map<String, Value>,
    base_score: i64,
}

impl<'a> BudouxParser<'a> {
    pub fn try_new_with_model(model: &'a Map<String, Value>) -> Result<Self, &'static str> {
        let mut sum = 0;
        for value in model.values() {
            if let Some(v) = value.as_object() {
                sum += v.values().fold(0, |sum, x| sum + x.as_i64().unwrap_or(0));
            }
        }
        let base_score = -sum / 2;
        Ok(Self { model, base_score })
    }

    pub fn parse_one(&self, sentence: &str) -> Option<usize> {
        if sentence.len() == 0 {
            return None;
        }
        self.parse_boundary(sentence)
    }

    fn get_score(&self, key: &str, sentence: &str, offset: i32, len: usize) -> Option<i64> {
        if offset < 0 {
            return None;
        }
        let mut characters: String = "".to_string();
        let mut iter = sentence.char_indices();
        let (_, ch) = iter.nth(offset as usize)?;
        characters.push(ch);

        let mut len = len - 1;
        while len > 0 {
            let (_, ch) = iter.next()?;
            characters.push(ch);
            len -= 1;
        }
        self.model.get(key)?.as_object()?.get(&characters)?.as_i64()
    }

    fn parse_boundary(&self, sentence: &str) -> Option<usize> {
        let mut i: i32 = 1;
        while i < sentence.chars().count() as i32 {
            let mut score = self.base_score;

            score += self.get_score("UW1", sentence, i - 3, 1).unwrap_or(0);
            score += self.get_score("UW2", sentence, i - 2, 1).unwrap_or(0);
            score += self.get_score("UW3", sentence, i - 1, 1).unwrap_or(0);
            score += self.get_score("UW4", sentence, i, 1).unwrap_or(0);
            score += self.get_score("UW5", sentence, i + 1, 1).unwrap_or(0);
            score += self.get_score("UW6", sentence, i + 2, 1).unwrap_or(0);
            score += self.get_score("BW1", sentence, i - 2, 2).unwrap_or(0);
            score += self.get_score("BW2", sentence, i - 1, 2).unwrap_or(0);
            score += self.get_score("BW3", sentence, i, 2).unwrap_or(0);
            score += self.get_score("TW1", sentence, i - 3, 3).unwrap_or(0);
            score += self.get_score("TW2", sentence, i - 2, 3).unwrap_or(0);
            score += self.get_score("TW3", sentence, i - 1, 3).unwrap_or(0);
            score += self.get_score("TW4", sentence, i, 3).unwrap_or(0);

            if score > 0 {
                return Some(
                    sentence
                        .chars()
                        .take(i as usize)
                        .fold(0, |sum, x| sum + x.len_utf8()),
                );
            }
            i += 1;
        }
        None
    }
}
