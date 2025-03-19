use super::basefn::midi_generator;
use crate::Score;

pub struct Channel {
    pub name: String,
    // score: Score,
    // pub score: Score,
    pub preset: String,
    pub volume: f32,
    pub n_poly: usize,
    pub pan: i8,
    pub be_modulated: bool,
}

impl Channel {
    // 构造函数，为某些字段设置初始值
    pub fn new(
        name: &str,
        // score: &str,
        preset: &str,
        volume: f32,
        n_poly: usize,
        pan: i8,
        be_modulated: bool,
    ) -> Self {
        Channel {
            name: name.to_string(),
            // score: midi_generator(score),
            preset: preset.to_string(),
            volume: volume, // 默认音量
            n_poly: n_poly, // 默认多音数量
            pan: pan,       // 默认声相（0 表示居中）
            be_modulated: be_modulated,
        }
    }
}
