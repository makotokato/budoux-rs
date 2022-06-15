//! ## Example
//!
//!```rust
//! use budoux_rs::Parser;
//! use serde_json::{Map, Value};
//!
//! const MODELS: &[u8; 10213] = include_bytes!("../models/ja-knbc.json");
//! let parsed: Value = serde_json::from_slice(MODELS).unwrap();
//! let model: Map<String, Value> = parsed.as_object().unwrap().clone();
//!
//! let parser = Parser::try_new_with_model(model).unwrap();
//! let result = parser.parse("今日はいい天気ですね。");
//! assert_eq!(result[0], 9);
//! assert_eq!(result[1], 15);
//! ```
//!

#![cfg_attr(not(any(test)), no_std)]

extern crate alloc;

mod parser;
mod unicode_block;

pub use crate::parser::Parser;
