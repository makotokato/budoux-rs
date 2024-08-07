use budoux_rs::*;

/*
#[test]
fn zh_cn_test() {
    const MODELS: &[u8; 26566] = include_bytes!("../models/zh-hans.json");
    let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

    let segmenter = BudoxSegmenter::try_new_with_model(&model).unwrap();
    let mut iter = segmenter.segment_str("今天天气晴朗。");
    assert_eq!(iter.next(), Some(12));
    assert_eq!(iter.next(), None);
}

#[test]
fn jp_knbc_test() {
    const MODELS: &[u8; 14434] = include_bytes!("../models/ja_knbc.json");
    let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

    let segmenter = BudoxSegmenter::try_new_with_model(&model).unwrap();
    let mut iter = segmenter.segment_str("今日はいい天気ですね。");
    assert_eq!(iter.next(), Some(9));
    assert_eq!(iter.next(), Some(15));
    assert_eq!(iter.next(), None);
}
*/

#[test]
fn jp_test() {
    const MODELS: &[u8; 17432] = include_bytes!("../models/ja.json");
    let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

    let segmenter = BudouxSegmenter::try_new_with_model(&model).unwrap();
    let mut iter = segmenter.segment_str("今日は天気です。");
    assert_eq!(iter.next(), Some(9));
    assert_eq!(iter.next(), None);

    iter = segmenter.segment_str("今日はいい天気ですね。");
    assert_eq!(iter.next(), Some(9));
    assert_eq!(iter.next(), Some(15));
    assert_eq!(iter.next(), None);
}
