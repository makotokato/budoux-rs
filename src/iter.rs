use crate::defines;
use crate::feature::*;
use alloc::string::String;
use core::str::Chars;
use serde_json::{Map, Value};

pub struct BudoxSegmenter<'a> {
    model: &'a Map<String, Value>,
    thres: i64,
}

impl<'a> BudoxSegmenter<'a> {
    pub fn try_new_with_model(model: &'a Map<String, Value>) -> Result<Self, &'static str> {
        Ok(Self { model, thres: 1000 })
    }

    pub fn segment_str(&'a self, input: &'a str) -> BudoxSegmenterIterator {
        BudoxSegmenterIterator::new(self.model, self.thres, input)
    }
}

pub struct BudoxSegmenterIterator<'a> {
    model: &'a Map<String, Value>,
    thres: i64,

    current: Chars<'a>,
    utf8_offset: usize,

    // state machine data
    w1: char,
    w2: char,
    w3: char,
    w4: char,
    w5: char,
    p1: char,
    p2: char,
    p3: char,
}

impl<'a> Iterator for BudoxSegmenterIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.w4 == char::REPLACEMENT_CHARACTER {
            return None;
        }

        let mut w1 = self.w1;
        let mut w2 = self.w2;
        let mut w3 = self.w3;
        let mut w4 = self.w4;
        let mut w5 = self.w5;
        let mut p1 = self.p1;
        let mut p2 = self.p2;
        let mut p3 = self.p3;

        while w4 != char::REPLACEMENT_CHARACTER {
            let w6 = self.current.next().unwrap_or(char::REPLACEMENT_CHARACTER);
            let feature = get_feature(w1, w2, w3, w4, w5, w6, p1, p2, p3);
            let mut score = 0;

            for f in feature.iter() {
                if let Some(v) = self.model.get(f) {
                    score += v.as_i64().unwrap_or(0);
                }
            }

            w1 = w2;
            w2 = w3;
            w3 = w4;
            w4 = w5;
            w5 = w6;
            p1 = p2;
            p2 = p3;
            let p = if score > 0 {
                defines::POSITIVE
            } else {
                defines::NEGATIVE
            };
            p3 = p;
            // w3 was current character
            self.utf8_offset += w3.len_utf8();

            if score > self.thres {
                // break opportunity

                // Save state machine
                self.w1 = w1;
                self.w2 = w2;
                self.w3 = w3;
                self.w4 = w4;
                self.w5 = w5;
                self.p1 = p1;
                self.p2 = p2;
                self.p3 = p3;

                return Some(self.utf8_offset - self.w3.len_utf8());
            }
        }

        // Rearch EOF
        self.w4 = w4;
        None
    }
}

impl<'a> BudoxSegmenterIterator<'a> {
    pub fn new(model: &'a Map<String, Value>, thres: i64, input: &'a str) -> Self {
        let mut iter = input.chars();
        let w4 = iter.next().unwrap_or(char::REPLACEMENT_CHARACTER);
        let w5 = iter.next().unwrap_or(char::REPLACEMENT_CHARACTER);

        Self {
            model,
            thres,
            current: iter,
            utf8_offset: 0,
            w1: char::REPLACEMENT_CHARACTER,
            w2: char::REPLACEMENT_CHARACTER,
            w3: char::REPLACEMENT_CHARACTER,
            w4,
            w5,
            p1: defines::UNKNOWN,
            p2: defines::UNKNOWN,
            p3: defines::UNKNOWN,
        }
    }
}
