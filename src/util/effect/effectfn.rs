use crate::ModulateParameters;
use crate::Timestamp;
use crate::SAMPLE_RATE;

pub fn fm_modulate(clock: Timestamp, params: &ModulateParameters) -> f32 {
    params.range * (params.frequency * clock as f32 / SAMPLE_RATE as f32).sin()
}
