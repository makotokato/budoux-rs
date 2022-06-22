use crate::defines;
use crate::unicode_block::UNICODE_BLOCK;
use alloc::format;
use alloc::string::{String, ToString};
use core::str::Chars;
use serde_json::{Map, Value};

fn get_unicode_block_index(input: char) -> String {
    if input == char::REPLACEMENT_CHARACTER {
        char::REPLACEMENT_CHARACTER.to_string()
    } else {
        let input = input as u32;
        format!("{:03}", bisection::bisect_right(&UNICODE_BLOCK, &input))
    }
}

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

    raw_feature: [(&'static str, String); 42],

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
            self.fill_feature(w1, w2, w3, w4, w5, w6, p1, p2, p3);
            let mut score = 0;

            for f in self
                .raw_feature
                .iter()
                .filter(|(_, v)| !v.contains(char::REPLACEMENT_CHARACTER))
                .map(|(k, v)| format!("{}:{}", k, v))
            {
                if let Some(v) = self.model.get(&f) {
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
            raw_feature: [
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
                ("", "".to_string()),
            ],
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

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    fn fill_feature(
        &mut self,
        w1: char,
        w2: char,
        w3: char,
        w4: char,
        w5: char,
        w6: char,
        p1: char,
        p2: char,
        p3: char,
    ) {
        let b1 = get_unicode_block_index(w1);
        let b2 = get_unicode_block_index(w2);
        let b3 = get_unicode_block_index(w3);
        let b4 = get_unicode_block_index(w4);
        let b5 = get_unicode_block_index(w5);
        let b6 = get_unicode_block_index(w6);

        self.raw_feature[0] = ("UP1", p1.to_string());
        self.raw_feature[1] = ("UP2", p2.to_string());
        self.raw_feature[2] = ("UP3", p3.to_string());
        self.raw_feature[3] = ("BP1", p1.to_string() + &p2.to_string());
        self.raw_feature[4] = ("BP2", p2.to_string() + &p3.to_string());
        self.raw_feature[5] = ("UW1", w1.to_string());
        self.raw_feature[6] = ("UW2", w2.to_string());
        self.raw_feature[7] = ("UW3", w3.to_string());
        self.raw_feature[8] = ("UW4", w4.to_string());
        self.raw_feature[9] = ("UW5", w5.to_string());
        self.raw_feature[10] = ("UW6", w6.to_string());
        self.raw_feature[11] = ("BW1", w2.to_string() + &w3.to_string());
        self.raw_feature[12] = ("BW2", w3.to_string() + &w4.to_string());
        self.raw_feature[13] = ("BW3", w4.to_string() + &w5.to_string());
        self.raw_feature[14] = ("TW1", w1.to_string() + &w2.to_string() + &w3.to_string());
        self.raw_feature[15] = ("TW2", w2.to_string() + &w3.to_string() + &w4.to_string());
        self.raw_feature[16] = ("TW3", w3.to_string() + &w4.to_string() + &w5.to_string());
        self.raw_feature[17] = ("TW4", w4.to_string() + &w5.to_string() + &w6.to_string());
        self.raw_feature[18] = ("UB1", b1.to_string());
        self.raw_feature[19] = ("UB2", b2.to_string());
        self.raw_feature[20] = ("UB3", b3.to_string());
        self.raw_feature[21] = ("UB4", b4.to_string());
        self.raw_feature[22] = ("UB5", b5.to_string());
        self.raw_feature[23] = ("UB6", b6.to_string());
        self.raw_feature[24] = ("BB1", b2.to_string() + &b3);
        self.raw_feature[25] = ("BB2", b3.to_string() + &b4);
        self.raw_feature[26] = ("BB3", b4.to_string() + &b5);
        self.raw_feature[27] = ("TB1", b1.to_string() + &b2 + &b3);
        self.raw_feature[28] = ("TB2", b2.to_string() + &b3 + &b4);
        self.raw_feature[29] = ("TB3", b3.to_string() + &b4 + &b5);
        self.raw_feature[30] = ("TB4", b4.to_string() + &b5 + &b6);
        self.raw_feature[31] = ("UQ1", p1.to_string() + &b1);
        self.raw_feature[32] = ("UQ2", p2.to_string() + &b2);
        self.raw_feature[33] = ("UQ3", p3.to_string() + &b3);
        self.raw_feature[34] = ("BQ1", p2.to_string() + &b2 + &b3);
        self.raw_feature[35] = ("BQ2", p2.to_string() + &b3 + &b4);
        self.raw_feature[36] = ("BQ3", p3.to_string() + &b2 + &b3);
        self.raw_feature[37] = ("BQ4", p3.to_string() + &b3 + &b4);
        self.raw_feature[38] = ("TQ1", p2.to_string() + &b1 + &b2 + &b3);
        self.raw_feature[39] = ("TQ2", p2.to_string() + &b2 + &b3 + &b4);
        self.raw_feature[40] = ("TQ3", p3.to_string() + &b1 + &b2 + &b3);
        self.raw_feature[41] = ("TQ4", p3.to_string() + &b2 + &b3 + &b4);
    }
}
