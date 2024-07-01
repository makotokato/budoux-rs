#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use budoux_rs::*;
    use test::Bencher;

    const STR: &str = "吾輩は猫である名前はまだ無いどこで生れたかとんと見当がつかぬ何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している吾輩はここで始めて人間というものを見たしかもあとで>聞くとそれは書生という人間中で一番獰悪な種族であったそうだこの書生というのは時々我々を捕えて煮て食うという話であるしかしその当時は何という考もなかったから別段恐しいとも思わなかったただ彼の掌に載せられてスーと持ち上げられた時何だかフワフワした感じがあったばかりである掌の上で少し落ちついて書生の顔を見たのがいわゆる人間というものの見始であろうこの時妙なものだと思った感じが今でも残っている第一毛をもって装飾されべきはずの顔がつるつるしてまるで薬缶だその後猫にもだいぶ逢ったがこんな片輪には一度も出会わした事がないのみならず顔の真中があまりに突起しているそうしてその穴の中から時々ぷうぷうと煙を吹くどうも咽せぽくて実に弱ったこれが人間の飲む煙草というものである事はようやくこの頃知った";

    #[bench]
    fn jp_iter(b: &mut Bencher) {
        const MODELS: &[u8; 17432] = include_bytes!("../models/ja.json");
        let parsed: serde_json::Value = serde_json::from_slice(MODELS).unwrap();
        let model: serde_json::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();

        let segmenter = BudoxSegmenter::try_new_with_model(&model).unwrap();
        b.iter(|| segmenter.segment_str(STR).count());
    }
}
