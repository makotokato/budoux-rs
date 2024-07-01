//! ## Example
//!
//!```rust
//! use budoux_rs::BudoxSegmenter;
//! use serde_json::{Map, Value};
//!
//! const MODELS: &[u8; 17432] = include_bytes!("../models/ja.json");
//! let parsed: Value = serde_json::from_slice(MODELS).unwrap();
//! let model: Map<String, Value> = parsed.as_object().unwrap().clone();
//!
//! let segmenter = BudoxSegmenter::try_new_with_model(&model).unwrap();
//! let mut iter = segmenter.segment_str("今日は天気です。");
//! assert_eq!(iter.next(), Some(9));
//! assert_eq!(iter.next(), None);
//! ```
//!

#![cfg_attr(not(any(test)), no_std)]

extern crate alloc;

mod iter;
mod parser;

pub use crate::iter::{BudoxSegmenter, BudoxSegmenterIterator};
