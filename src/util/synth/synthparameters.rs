use crate::FTimestamp;
use crate::ModulateParameters;
use crate::MAX_POLY;
use crate::SAMPLE_RATE;
use std::cell::Cell;

pub struct SynthParameters {
    pub t: [Cell<FTimestamp>; MAX_POLY],
    pub delta_t: [FTimestamp; MAX_POLY],
    pub frequency: f32,
    pub volume: f32,
    pub preset: String,
    pub n_poly: usize,
    pub be_modulated: bool,
    pub modulate: ModulateParameters, // modulate: ModulateParameters,
}

impl SynthParameters {
    pub fn new(
        frequency: f32,
        volume: f32,
        preset: &str,
        n_poly: usize,
        be_modulated: bool,
    ) -> Self {
        if frequency == 0.0 {
            panic!("Division by zero frequency");
        }
        let range = 0.01;
        let mut delta_time: [FTimestamp; MAX_POLY] = [frequency / SAMPLE_RATE as f32; MAX_POLY];
        let t: [Cell<f32>; MAX_POLY] = std::array::from_fn(|_| Cell::new(0.0));

        let preset = String::from(preset);
        for i in 0..n_poly {
            delta_time[i] *= 1.0 + range - 2.0 * range * ((i + 1) / n_poly) as f32;
            t[i].set(0.2 * (i as FTimestamp) / n_poly as FTimestamp);
        }
        SynthParameters {
            t: t,
            delta_t: delta_time,
            frequency,
            volume,
            preset,
            n_poly,
            be_modulated: be_modulated,
            modulate: ModulateParameters {
                frequency: 50.0,
                range: 0.01,
            },
        }
    }
}
