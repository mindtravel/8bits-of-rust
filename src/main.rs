use std::sync::LazyLock;
use std::collections::HashMap;


extern crate libm;
extern crate rand;
use rand::Rng;

mod util;
use util::basefn::generate_wav;
use util::basefn::load_wav;

use util::basetype::Level;
use util::basetype::Timebase;
use util::basetype::Timestamp;
use util::basetype::FTimestamp;
use util::basetype::NoteType;
use util::basetype::Note;
use util::basetype::Midi;
use util::basetype::Score;
use util::basetype::ModulateParameters;

use util::channel::Channel;
use util::synthparameters::SynthParameters;
use util::song::SONG;

use util::baseconst::N_TBASE;
use util::baseconst::SONG_LEN;
use util::baseconst::SAMPLE_RATE; // 采样率
use util::baseconst::BPM;
use util::baseconst::T_BEAT;
use util::baseconst::T_BASE;
use util::baseconst::N_CHAN;
// use util::baseconst::LOOP_TIMES;
use util::baseconst::LOOP_TIMES;
use util::baseconst::MAX_POLY;
use util::baseconst::FREQ_DATA;

// use winapi::um::winuser::{FindWindowA, ShowWindow, SW_HIDE};

// fn hide_window() {
//     unsafe {
//         let window_title = "你的窗口标题"; // 替换为你的窗口标题
//         let hwnd = FindWindowA(std::ptr::null(), window_title.as_ptr() as *const i8);

//         if !hwnd.is_null() {
//             // 隐藏窗口
//             ShowWindow(hwnd, SW_HIDE);
//         }
//     }

//     // 你的程序逻辑
//     println!("窗口已隐藏");
// }
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

fn fm_modulate(clock: Timestamp, params: &ModulateParameters) -> f32 {
    params.range * (params.frequency * clock as f32 / SAMPLE_RATE as f32).sin()
}

fn wave_generator(preset: &str, phase: FTimestamp) -> f32 {
    let ret = match preset {
        "saw" => saw_wave(phase),
        "square" => square_wave(phase),
        "triangle" => triangle_wave(phase),
        "spike" => spike_wave(phase),
        "noise" => noise_wave(phase),
        _ => panic!("Cant find synthesiser preset {}", preset),
    };
    ret
}

fn multi_generator(preset: &str, _phase: [FTimestamp; MAX_POLY], n_poly: usize) -> f32 {
    // println!("{}",index);
    let mut ret = 0.0;
    let phase = _phase;
    for i in 0..n_poly {
        ret += wave_generator(&preset, phase[i] % 2.0 as f32);
    }
    ret
}

fn synth(params: &SynthParameters, clock: Timestamp) -> Level {
    let mut phase: [FTimestamp; MAX_POLY] = [0.0; MAX_POLY];
    for i in 0..params.n_poly{
        params.t[i].set(params.t[i].get() + params.delta_t[i]);
        if params.be_modulated == true{
            params
                .t[i]
                .set(params.t[i].get() + fm_modulate(clock, &params.modulate) * params.delta_t[i]);   
        }
        phase[i] = params.t[i].get() % 2.0;
    }

    // phase = fm_modulate(clock, params.frequency);
    let ret = multi_generator(&params.preset, phase, params.n_poly);
    ((ret * params.volume)/params.n_poly as f32) as Level
}

fn mixer(song: &LazyLock<[Channel; N_CHAN]>) -> Vec<Level> {
    let mut clock = 0 as Timestamp;
    let full_samples = (SAMPLE_RATE as f32 * T_BEAT) as Timestamp;
    let full_samples = (SAMPLE_RATE as f32 * T_BASE) as Timestamp;
    let mut sample: Vec<Level> = Vec::new();
    let mut synth_parameters: HashMap<usize, SynthParameters> = HashMap::new();

    let mut idx: Timebase = 0;
    while idx < SONG_LEN {
        let mut channel_idx = 0;

        while (channel_idx) < N_CHAN {
            // 初始化音轨设置
            if let Some(midis) = song[channel_idx].score.get_vec(&idx) {
                for midi in midis {
                    if midi.typ == START!() as NoteType {
                        synth_parameters.insert(
                            channel_idx * 128 + midi.note as usize,
                            SynthParameters::new(
                                FREQ_DATA[midi.note as usize],
                                song[channel_idx].volume,
                                &song[channel_idx].preset,
                                song[channel_idx].n_poly,
                                song[channel_idx].be_modulated,
                            ),
                        );
                    }
                    if midi.typ == END!() as NoteType {
                        synth_parameters.remove(&(channel_idx * 128 + midi.note as usize));
                    }
                }
            }
            channel_idx += 1;
        }
        for _i in 0..full_samples {
            let mut res:Level = 0;
            for params in synth_parameters.values() {
                res += synth(params, clock);
            }
            sample.push(res);
            clock += 1;
        }

        idx += 1;
    }
    sample
    // println!("{}", f);
}



fn main() {
    let name = "my_wave";
    let sample = mixer(&SONG);
    generate_wav(name, sample);
    load_wav(name);
}
