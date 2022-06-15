#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use budoux_rs::Parser;
    use test::Bencher;

    const STR: &str = "吾輩は猫である。名前はまだない。";

    #[bench]
    fn jp_iter(b: &mut Bencher) {
        const MODELS: &[u8; 10213] = include_bytes!("../models/ja-knbc.json");
        let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
        let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

        let parser = Parser::try_new_with_model(model).unwrap();
        b.iter(|| parser.parse(STR).len())
    }
}
