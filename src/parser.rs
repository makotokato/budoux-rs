use crate::feature::*;
use alloc::string::String;
use alloc::vec::Vec;
use serde_json::{Map, Value};

pub struct Parser {
    model: Map<String, Value>,
    thres: i64,
}

impl Parser {
    pub fn try_new_with_model(model: Map<String, Value>) -> Result<Self, &'static str> {
        Ok(Self { model, thres: 1000 })
    }

    pub fn parse(&self, sentence: &str) -> Vec<usize> {
        let mut p1 = 'U';
        let mut p2 = 'U';
        let mut p3 = 'U';
        let mut chunks = Vec::new();
        let mut utf8_offset = 0;

        let mut current = sentence.chars();
        let mut w1 = char::REPLACEMENT_CHARACTER;
        let mut w2 = char::REPLACEMENT_CHARACTER;
        let mut w3 = char::REPLACEMENT_CHARACTER;
        let mut w4 = current.next().unwrap_or(char::REPLACEMENT_CHARACTER);
        let mut w5 = current.next().unwrap_or(char::REPLACEMENT_CHARACTER);

        while w4 != char::REPLACEMENT_CHARACTER {
            let w6 = current.next().unwrap_or(char::REPLACEMENT_CHARACTER);
            let feature = get_feature(w1, w2, w3, w4, w5, w6, p1, p2, p3);
            let mut score = 0;

            for f in feature.iter() {
                if let Some(v) = self.model.get(f) {
                    score += v.as_i64().unwrap_or(0);
                }
            }
            if score > self.thres {
                // break opportunity
                chunks.push(utf8_offset);
            }

            utf8_offset += w4.len_utf8();
            w1 = w2;
            w2 = w3;
            w3 = w4;
            w4 = w5;
            w5 = w6;
            p1 = p2;
            p2 = p3;
            let p = if score > 0 { 'B' } else { 'O' };
            p3 = p;
        }

        chunks
    }
}
