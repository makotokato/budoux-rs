use crate::unicode_block::UNICODE_BLOCK;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bisection;

fn get_unicode_block_index(input: char) -> String {
    if input == char::REPLACEMENT_CHARACTER {
        char::REPLACEMENT_CHARACTER.to_string()
    } else {
        let input = input as u32;
        format!("{:03}", bisection::bisect_right(&UNICODE_BLOCK, &input))
    }
}

fn get_feature(
    w1: char,
    w2: char,
    w3: char,
    w4: char,
    w5: char,
    w6: char,
    p1: char,
    p2: char,
    p3: char,
) -> Vec<String> {
    let b1 = get_unicode_block_index(w1);
    let b2 = get_unicode_block_index(w2);
    let b3 = get_unicode_block_index(w3);
    let b4 = get_unicode_block_index(w4);
    let b5 = get_unicode_block_index(w5);
    let b6 = get_unicode_block_index(w6);

    let raw_feature = BTreeMap::from([
        ("UP1", p1.to_string()),
        ("UP2", p2.to_string()),
        ("UP3", p3.to_string()),
        ("BP1", p1.to_string() + &p2.to_string()),
        ("BP2", p2.to_string() + &p3.to_string()),
        ("UW1", w1.to_string()),
        ("UW2", w2.to_string()),
        ("UW3", w3.to_string()),
        ("UW4", w4.to_string()),
        ("UW5", w5.to_string()),
        ("UW6", w6.to_string()),
        ("BW1", w2.to_string() + &w3.to_string()),
        ("BW2", w3.to_string() + &w4.to_string()),
        ("BW3", w4.to_string() + &w5.to_string()),
        ("TW1", w1.to_string() + &w2.to_string() + &w3.to_string()),
        ("TW2", w2.to_string() + &w3.to_string() + &w4.to_string()),
        ("TW3", w3.to_string() + &w4.to_string() + &w5.to_string()),
        ("TW4", w4.to_string() + &w5.to_string() + &w6.to_string()),
        ("UB1", b1.to_string()),
        ("UB2", b2.to_string()),
        ("UB3", b3.to_string()),
        ("UB4", b4.to_string()),
        ("UB5", b5.to_string()),
        ("UB6", b6.to_string()),
        ("BB1", b2.to_string() + &b3),
        ("BB2", b3.to_string() + &b4),
        ("BB3", b4.to_string() + &b5),
        ("TB1", b1.to_string() + &b2 + &b3),
        ("TB2", b2.to_string() + &b3 + &b4),
        ("TB3", b3.to_string() + &b4 + &b5),
        ("TB4", b4.to_string() + &b5 + &b6),
        ("UQ1", p1.to_string() + &b1),
        ("UQ2", p2.to_string() + &b2),
        ("UQ3", p3.to_string() + &b3),
        ("BQ1", p2.to_string() + &b2 + &b3),
        ("BQ2", p2.to_string() + &b3 + &b4),
        ("BQ3", p3.to_string() + &b2 + &b3),
        ("BQ4", p3.to_string() + &b3 + &b4),
        ("TQ1", p2.to_string() + &b1 + &b2 + &b3),
        ("TQ2", p2.to_string() + &b2 + &b3 + &b4),
        ("TQ3", p3.to_string() + &b1 + &b2 + &b3),
        ("TQ4", p3.to_string() + &b2 + &b3 + &b4),
    ]);
    raw_feature
        .iter()
        .filter(|(_, v)| !v.contains(char::REPLACEMENT_CHARACTER))
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect()
}

pub struct Parser {
    model: serde_json::Map<String, serde_json::Value>,
}

impl Parser {
    pub fn try_new_with_model(
        model: serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, &'static str> {
        Ok(Self { model })
    }

    pub fn parse(self, sentence: &str) -> Vec<usize> {
        let sentence = sentence.to_string();
        let mut p1 = 'U';
        let mut p2 = 'U';
        let mut p3 = 'U';
        let mut i = 0;
        let thres = 1000;
        let mut chunks = Vec::new();
        let mut utf8_offset = 0;

        let mut w1 = char::REPLACEMENT_CHARACTER;
        let mut w2 = char::REPLACEMENT_CHARACTER;
        let mut w3 = char::REPLACEMENT_CHARACTER;
        let mut w4 = sentence
            .chars()
            .nth(0)
            .unwrap_or(char::REPLACEMENT_CHARACTER);
        let mut w5 = sentence
            .chars()
            .nth(1)
            .unwrap_or(char::REPLACEMENT_CHARACTER);

        while i < sentence.chars().count() {
            let w6 = sentence
                .chars()
                .nth(i + 2)
                .unwrap_or(char::REPLACEMENT_CHARACTER);
            let feature = get_feature(w1, w2, w3, w4, w5, w6, p1, p2, p3);
            let mut score = 0;

            for f in feature.iter() {
                if let Some(v) = self.model.get(f) {
                    score += v.as_i64().unwrap_or(0);
                }
            }
            if score > thres {
                // break opportunity
                chunks.push(utf8_offset);
            }

            utf8_offset += w4.len_utf8();
            w1 = w2;
            w2 = w3;
            w3 = w4;
            w4 = w5;
            w5 = w6;
            let p = if score > 0 { 'B' } else { 'O' };
            p1 = p2;
            p2 = p3;
            p3 = p;
            i += 1;
        }

        chunks
    }
}
