use crate::unicode_block::UNICODE_BLOCK;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bisection;

const MODELS: &[u8; 10213] = include_bytes!("../models/ja-knbc.json");

fn get_unicode_block_index(input: char) -> String {
    if input == char::REPLACEMENT_CHARACTER {
        char::REPLACEMENT_CHARACTER.to_string()
    } else {
        let input = input as u32;
        format!("{:03}", bisection::bisect_right(&UNICODE_BLOCK, &input))
    }
}

#[allow(dead_code)]
struct Parser {
    model: serde_json::Map<String, serde_json::Value>,
}

pub fn get_feature(
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

    let mut raw_feature = BTreeMap::new();
    raw_feature.insert("UP1", p1.to_string());
    raw_feature.insert("UP2", p2.to_string());
    raw_feature.insert("UP3", p3.to_string());
    raw_feature.insert("BP1", p1.to_string() + &p2.to_string());
    raw_feature.insert("BP2", p2.to_string() + &p3.to_string());
    raw_feature.insert("UW1", w1.to_string());
    raw_feature.insert("UW2", w2.to_string());
    raw_feature.insert("UW3", w3.to_string());
    raw_feature.insert("UW4", w3.to_string());
    raw_feature.insert("UW5", w5.to_string());
    raw_feature.insert("UW6", w6.to_string());
    raw_feature.insert("BW1", w2.to_string() + &w3.to_string());
    raw_feature.insert("BW2", w3.to_string() + &w4.to_string());
    raw_feature.insert("BW3", w4.to_string() + &w5.to_string());
    raw_feature.insert("TW1", w1.to_string() + &w2.to_string() + &w3.to_string());
    raw_feature.insert("TW2", w2.to_string() + &w3.to_string() + &w4.to_string());
    raw_feature.insert("TW3", w3.to_string() + &w4.to_string() + &w5.to_string());
    raw_feature.insert("TW4", w4.to_string() + &w5.to_string() + &w6.to_string());
    raw_feature.insert("UB1", b1.to_string());
    raw_feature.insert("UB2", b2.to_string());
    raw_feature.insert("UB3", b3.to_string());
    raw_feature.insert("UB4", b4.to_string());
    raw_feature.insert("UB5", b5.to_string());
    raw_feature.insert("UB6", b6.to_string());
    raw_feature.insert("BB1", b2.to_string() + &b3);
    raw_feature.insert("BB2", b3.to_string() + &b4);
    raw_feature.insert("BB3", b4.to_string() + &b5);
    raw_feature.insert("TB1", b1.to_string() + &b2 + &b3);
    raw_feature.insert("TB2", b2.to_string() + &b3 + &b4);
    raw_feature.insert("TB3", b3.to_string() + &b4 + &b5);
    raw_feature.insert("TB4", b4.to_string() + &b5 + &b6);
    raw_feature.insert("UQ1", p1.to_string() + &b1);
    raw_feature.insert("UQ2", p2.to_string() + &b2);
    raw_feature.insert("UQ3", p3.to_string() + &b3);
    raw_feature.insert("BQ1", p2.to_string() + &b2 + &b3);
    raw_feature.insert("BQ2", p2.to_string() + &b3 + &b4);
    raw_feature.insert("BQ3", p3.to_string() + &b2 + &b3);
    raw_feature.insert("BQ4", p3.to_string() + &b3 + &b4);
    raw_feature.insert("TQ1", p2.to_string() + &b1 + &b2 + &b3);
    raw_feature.insert("TQ2", p2.to_string() + &b2 + &b3 + &b4);
    raw_feature.insert("TQ3", p3.to_string() + &b1 + &b2 + &b3);
    raw_feature.insert("TQ4", p3.to_string() + &b2 + &b3 + &b4);
    raw_feature.retain(|_, v| !v.contains(char::REPLACEMENT_CHARACTER));

    let mut result = Vec::new();
    for (key, value) in raw_feature.iter() {
        result.push(format!("{}:{}", key, value));
    }
    result
}

pub fn parse(sentence: &String) -> Vec<String> {
    let mut p1 = 'U';
    let mut p2 = 'U';
    let mut p3 = 'U';
    let mut i = 0;
    let thres = 1000;
    let mut chunks = Vec::new();
    //chunks.push(sentence.chars().nth(0).unwrap().to_string());

    let parsed: serde_json::Value = serde_json::from_slice(MODELS).expect("JSON syntax error");
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();
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
        w1 = w2;
        w2 = w3;
        w3 = w4;
        w4 = w5;
        w5 = w6;
        let mut score = 0;

        for f in feature.iter() {
            if let Some(v) = model.get(f) {
                score = score + v.as_i64().unwrap_or(0);
            }
        }
        if score > thres {
            chunks.push(sentence.chars().nth(i).unwrap().to_string());
        } else {
            let last_str = chunks.pop().unwrap_or("".to_string());
            chunks.push(last_str + &sentence.chars().nth(i).unwrap().to_string());
        }
        let p = if score > 0 { 'B' } else { 'O' };
        p1 = p2;
        p2 = p3;
        p3 = p;
        i = i + 1;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let result = parse(&"今日は天気です。".to_string());
        assert_eq!(result[0], "今日は");
        assert_eq!(result[1], "天気です。");
    }
}
