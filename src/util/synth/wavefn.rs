use crate::FTimestamp;
use crate::MAX_POLY;
use rand::Rng;

fn square_wave(phase: FTimestamp) -> f32 {
    // println!("{}",index);
    if phase - 1.0 < 0.0 {
        127.0
    } else {
        -128.0
    }
}

fn spike_wave(phase: FTimestamp) -> f32 {
    if phase - 1.0 < 0.6 {
        127.0
    } else {
        -128.0
    }
}

fn saw_wave(phase: FTimestamp) -> f32 {
    255.5 * (phase - 1.0)
}

fn triangle_wave(phase: FTimestamp) -> f32 {
    if phase < 0.5 {
        127.0 - 255.0 * (0.5 - phase)
    } else if phase > 1.5 {
        127.0 - 255.0 * (2.5 - phase)
    } else {
        127.0 - 255.0 * (phase - 0.5)
    }
}

fn noise_wave(_phase: FTimestamp) -> f32 {
    let mut rng = rand::rng();
    -128.0 + 255.0 * ((rng.random_bool(0.5) as i32) as f32)
}

fn wave_generator(preset: &str, phase: FTimestamp) -> f32 {
    let preset = preset.trim().to_lowercase();
    let ret = match preset.as_str() {
        "saw" => saw_wave(phase),
        "square" => square_wave(phase),
        "triangle" => triangle_wave(phase),
        "spike" => spike_wave(phase),
        "noise" => noise_wave(phase),
        _ => panic!("Cant find synthesiser preset {}", preset),
    };
    ret
}

pub fn multi_generator(preset: &str, _phase: [FTimestamp; MAX_POLY], n_poly: usize) -> f32 {
    // println!("{}",index);
    let mut ret = 0.0;
    let phase = _phase;
    for i in 0..n_poly {
        ret += wave_generator(&preset, phase[i] % 2.0 as f32);
    }
    ret
}
