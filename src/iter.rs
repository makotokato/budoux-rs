use crate::parser::BudouxParser;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::Chars;
use serde_json::{Map, Value};

pub struct BudouxSegmenter<'a> {
    model: &'a Map<String, Value>,
}

impl<'a> BudouxSegmenter<'a> {
    pub fn try_new_with_model(model: &'a Map<String, Value>) -> Result<Self, &'static str> {
        Ok(Self { model })
    }

    pub fn segment_str(&'a self, input: &'a str) -> BudouxSegmenterIterator {
        BudouxSegmenterIterator::try_new(self.model, input).unwrap()
    }
}

pub struct BudouxSegmenterIterator<'a> {
    parser: BudouxParser<'a>,
    input: &'a str,
    last_index: usize,
}

impl<'a> Iterator for BudouxSegmenterIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.parser.parse_one(&self.input[self.last_index..]) {
            self.last_index += index;
            return Some(self.last_index);
        }
        None
    }
}

impl<'a> BudouxSegmenterIterator<'a> {
    pub fn try_new(model: &'a Map<String, Value>, input: &'a str) -> Option<Self> {
        let parser = BudouxParser::try_new_with_model(model).ok()?;

        Some(Self {
            parser,
            input,
            last_index: 0,
        })
    }
}
