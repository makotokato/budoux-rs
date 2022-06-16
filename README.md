# budoux-rs

A test implementation of Google's [budox](https://github.com/google/budoux) by Rust.

## How to use
```rust
use budoux_rs::BudoxSegmenter;
use serde_json::{Map, Value};

const MODELS: &[u8; 10213] = include_bytes!("models/ja-knbc.json");
let parsed: Value = serde_json::from_slice(MODELS).unwrap();
let model: Map<String, Value> = parsed.as_object().unwrap().clone();

let segmenter = BudoxSegmenter::try_new_with_model(&model).unwrap();
let mut iter = segmenter.segment_str("今日はいい天気ですね。");
assert_eq!(iter.next(), Some(9));
assert_eq!(iter.next(), Some(15));
assert_eq!(iter.next(), None);
```
