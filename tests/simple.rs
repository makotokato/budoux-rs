use budoux_rs::Parser;

#[test]
fn jp_test() {
    const MODELS: &[u8; 10213] = include_bytes!("../models/ja-knbc.json");
    let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

    let parser = Parser::try_new_with_model(model).unwrap();
    let result = parser.parse("今日は天気です。");
    assert_eq!(result[0], 9);
}

#[test]
fn zh_cn_test() {
    const MODELS: &[u8; 26566] = include_bytes!("../models/zh-hans.json");
    let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
    let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

    let parser = Parser::try_new_with_model(model).unwrap();
    let result = parser.parse("今天天气晴朗。");
    assert_eq!(result[0], 12);
}
