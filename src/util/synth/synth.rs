use crate::fm_modulate;
use crate::multi_generator;
use crate::FTimestamp;
use crate::Level;
use crate::SynthParameters;
use crate::Timestamp;
use crate::MAX_POLY;

pub fn synth(params: &SynthParameters, clock: Timestamp) -> Level {
    let mut phase: [FTimestamp; MAX_POLY] = [0.0; MAX_POLY];
    for i in 0..params.n_poly {
        params.t[i].set(params.t[i].get() + params.delta_t[i]);
        if params.be_modulated == true {
            params.t[i]
                .set(params.t[i].get() + fm_modulate(clock, &params.modulate) * params.delta_t[i]);
        }
        phase[i] = params.t[i].get() % 2.0;
    }

    // phase = fm_modulate(clock, params.frequency);
    let ret = multi_generator(&params.preset, phase, params.n_poly);
    ((ret * params.volume) / params.n_poly as f32) as Level
}
