use hound;
use multimap::MultiMap;
use rodio::{Decoder, OutputStream, Sink}; //, source::Source};
use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

extern crate libm;
extern crate rand;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::env;
use std::fs;

type Level = i8;
type Timebase = u32;
type Timestamp = u32;
type FTimestamp = f32;
type NoteType = i8;
type Note = u8;

const N_TBASE: u32 = 4;
const SONG_LEN: u32 = (16 + 64 + 64 + 64 + 8) * 2;
const SAMPLE_RATE: u32 = 44100; // 采样率
const BPM: u32 = 145;
const T_BEAT: f32 = 60.0 / BPM as f32;
const T_BASE: f32 = 60.0 / (BPM*N_TBASE) as f32;
const N_CHAN: usize = 5;
// const LOOP_TIMES:i16 = 8;
const LOOP_TIMES: u16 = u16::MAX;
const MAX_POLY: usize = 8;


const FREQ_DATA: [f32; 86] = [
    0.0, 
    32.7032, 34.6478, 36.7081, 38.8909, 41.2034, 43.6535, 46.2493, 48.9995, 51.9131, 55.0000, 58.2705, 61.7354,
    65.4064, 69.2957, 73.4162, 77.7817, 82.4069, 87.3071, 92.4986, 97.9989, 103.826, 110.000, 116.541, 123.471,
    130.813, 138.591, 146.832, 155.563, 164.814, 174.614, 184.997, 195.998, 207.652, 220.000, 233.082, 246.942,
    261.635, 277.183, 293.665, 311.127, 329.628, 349.228, 369.994, 391.995, 415.305, 440.000, 466.164, 493.883,
    523.251, 554.365, 587.330, 622.254, 659.256, 698.456, 739.989, 783.990, 830.609, 880.000, 932.328, 987.767,
    1046.502, 1108.732, 1174.658, 1244.508, 1318.510, 1396.913, 1479.978, 1567.982, 1661.219, 1760.000, 1864.659, 1975.533,
    2093.005, 2217.461, 2349.325, 2489.026, 2637.027, 2793.826, 2959.957, 3135.963, 3322.438, 3520.000, 3729.307, 3951.066, 4186.009,
];

macro_rules! START {
    () => {
        1
    };
}
macro_rules! END {
    () => {
        2
    };
}

struct Midi {
    note: Note,
    typ: NoteType,
}

type Pattern = MultiMap<Timebase, Midi>;

struct Channel {
    name: String,
    // pattern: Pattern,
    pattern: Pattern,
    preset: String,
    volume: f32,
    n_poly: usize,
    pan: i8,
    be_modulated: bool,
}

impl Channel {
    // 构造函数，为某些字段设置初始值
    fn new(name: &str, pattern: &str, preset: &str, volume: f32, n_poly: usize, pan:i8, be_modulated: bool) -> Self {
        Channel {
            name: name.to_string(),
            pattern: midi_generator(pattern),
            preset: preset.to_string(),
            volume: volume, // 默认音量
            n_poly: n_poly,   // 默认多音数量
            pan: pan,      // 默认声相（0 表示居中）
            be_modulated: be_modulated
        }
    }
}

struct ModulateParameters {
    frequency: f32,
    range: f32,
}

struct SynthParameters {
    t: [Cell<FTimestamp>; MAX_POLY],
    delta_t: [FTimestamp; MAX_POLY],
    frequency: f32,
    volume: f32,
    preset: String,
    n_poly: usize,
    be_modulated: bool,
    modulate: ModulateParameters
    // modulate: ModulateParameters,
}

impl SynthParameters {
    fn new(frequency: f32, volume: f32, preset: &str, n_poly: usize, be_modulated: bool) -> Self {
        if frequency == 0.0 {
            panic!("Division by zero frequency");
        }
        let range = 0.01;
        let mut delta_time: [FTimestamp; MAX_POLY] = [frequency / SAMPLE_RATE as f32; MAX_POLY]; 
        let t: [Cell<f32>; MAX_POLY] = std::array::from_fn(|_| Cell::new(0.0));
        
        let preset = String::from(preset);
        for i in 0..n_poly{
            delta_time[i] *= 1.0 + range - 2.0 * range * ((i + 1) / n_poly) as f32; 
            t[i].set(0.2 * (i as FTimestamp) / n_poly as FTimestamp) ; 
        }
        SynthParameters {
            t: t,
            delta_t: delta_time,
            frequency,
            volume,
            preset,
            n_poly,
            be_modulated: be_modulated, 
            modulate: ModulateParameters{
                frequency:50.0, 
                range: 0.01}
        }
    }
}

fn midi_generator(note: &str) -> Pattern {
    let mut tbase: Timebase = 0;
    let mut idx_vec: Vec<Note> = Vec::new();
    let mut pattern: Pattern = Pattern::new();
    let mut idx_tmp: Note = 0;
    let mut need_reset = false;
    for byte in note.bytes() {
        match byte {
            b'(' => {
                need_reset = true;
                for idx in &idx_vec{
                    pattern.insert(
                        tbase,
                        Midi {
                            note: *idx,
                            typ: START!(),
                        },
                    );
                }
            }
            b')' => {
                for idx in &idx_vec{
                    pattern.insert(
                        tbase,
                        Midi {
                            note: *idx,
                            typ: END!(),
                        },
                    );
                }            
            }
            b'-' => tbase += 2,
            b'=' => tbase += 1,
            b'A'..=b'G' => {
                idx_tmp = (byte - 4) % 7;
                idx_tmp = 2 * idx_tmp - (idx_tmp > 2) as Note;
            }
            b'#' => idx_tmp += 1,
            b'b' => idx_tmp -= 1,
            b'0'..=b'9' => {
                if need_reset == true{
                    idx_vec.clear();
                    need_reset = false;
                }
                idx_vec.push(idx_tmp + (byte - b'1') * 12 + 2);
            }
            _ => {}
        }
    }
    pattern
}

// struct Arrangement{

// }

use std::sync::LazyLock;
static CHANNELS: LazyLock<[Channel; N_CHAN]> = LazyLock::new(|| {
    [
        Channel::new(
            "1",
            
            "
        --------|------C4(-),D4(-),||

        D#4(-----),G4(-),F4(-),D#4(-),|F4(-),D4(-----),A#3(-),C4(-),|
        D4(-----),F4(-),D#4(-),D4(-),|D#4(-),C4(-----),D#4(-),F4(-),|
        G4(-----),A#4(-),G#4(-),G4(-),|G#4(-),F4(-----),C4(-),D4(-),|
        D#4(-----),F4(-),D#4(-),C4(-),|D4(------),C4(-),D4(-),||

        D#4(-----),G4(-),F4(-),D#4(-),|F4(-),D4(-----),A#3(-),C4(-),|
        D4(-----),F4(-),D#4(-),D4(-),|D#4(-),C4(-----),D#4(-),F4(-),|
        G4(-----),A#4(-),G#4(-),G4(-),|G#4(-),F4(-----),C4(-),D4(-),|
        D#4(-----),F4(-),D#4(-),C4(-),|G4(------),F4(-),G4(-),||

        G#4(------),D4(-),D#4(-)|F4(------),C4(-),D4(-)|
        D#4(------),C4(-),G3(-)|C4(-)D4(-)C4(-)D4(-)D#4(--)G4(-)G#4(-)||
        C5(-----)G#4(--)A#4(-)|D5(-----)G#4(--)A#4(-)|
        C4(=)D#4(=)F#4(=)A4(=)C5(=)D#5(=)F#5(=)A5(=)C6(=)A5(=)F#5(=)D#5(=)C5(=)A4(=)F#4(=)D#4(=)|D4(--------)||B3(--------)||
            ",
            "saw",
            // 0.0,
            0.065,
            1,
            0,
            true,
        ),
        Channel::new(
            "2",
            "
        C6D#6-(-)-(-)-(-)-(-)|-(-)-(-)-(-)-(-)||
        C6D#6-(-)-(-)-(-)-(-)|A#5D6-(-)-(-)-(-)-(-)|
         -(-)-(-)-(-)-(-)|G5C6-(-)-(-)-(-)-(-)|
        A#5D#6-(-)-(-)-(-)-(-),|A#5D6-(-)-(-)-(-)-(-)|
        G5C6-(-)-(-)-(-)-(-)|G5B5-(-)-(-)-(-)-(-)||
        C6D#6-(-)-(-)-(-)-(-)|A#5D6-(-)-(-)-(-)-(-)|
         -(-)-(-)-(-)-(-)|G5C6-(-)-(-)-(-)-(-)|
        A#5D#6-(-)-(-)-(-)-(-),|A#5D6-(-)-(-)-(-)-(-)|
        G5C6-(-)-(-)-(-)-(-)|G5B5-(-)-(-)-(-)-(-)||
        A#5D6-(-)-(-)----|G5A#5-(-)-(-)----|
        G#5C6-(-)-(-)----|D#5G5-(-)-(-)----|
        F5G#5C6-(-)-(=)=(--)--|G5A#5D6-(-)-(=)=(--)--||
        A#5D6G6(--)-----G5B5D6(=)=|(-)-B5D6F6(=)=G5B5D6(-----)||F5G#5B5(=)=G#5B5F5(-)-F5G#5B5(-----)||
            ",
            "square",
            // 0.0,
            0.05,
            1,
            0,
            true,
        ),
        Channel::new(
            "3",

            "
        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),||

        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),||

        D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||

        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),||

        D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||

        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-)||

        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),|
        G#2(---=)G3(=-)A#3(-)G#3(=)D3(=-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||
            ",
            "triangle",
            0.6,
            // 0.0, 
            1,
            0,
            false
        ),
        Channel::new(
            "4",
            "
        --------|--------||
        --------|-F5(-),D#5(=),D5(=),D#5(-),F5(-),---|--------|-D#5(-),D5(=),C5(=),D5(-),D#5(-),---|
        --------|-G#5(-),G5(=),F5(=),G5(-),G#5(-),---|--------|-G5(=),F5(=),D#5(-),F5(-),D5(-),---||
        --------|-F5(-),D#5(=),D5(=),D#5(-),F5(-),---|--------|-D#5(-),D5(=),C5(=),D5(-),D#5(-),---|
        --------|-G#5(-),G5(=),F5(=),G5(-),G#5(-),---|--------|-F5(=),D#5(=),D5(-),D#5(-),C5(--),B4(--)||
        --D5(-)C5(-)F5(--)--|--C5(-)B4(-)D5(--)--|--G#4(-)G4(-)D5(-)B4(=)C5(=)G4(--)|G5(-),G#5(-),G5(-),F#5(-),G5(--)--|
        --------|--------|
        C4(=)D#4(=)F#4(=)A4(=)C5(=)D#5(=)F#5(=)A5(=)C6(=)A5(=)F#5(=)D#5(=)C5(=)A4(=)F#4(=)D#4(=)|

            ",
            // G5B5D6(--------)||
            "spike",
            0.1,
            // 0.0, 
            1,  
            0,
            true
        ),
        Channel::new(
            "5",
            "C2
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=||
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=||
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=||
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=
        -(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=-(=)=||
            ",
            "noise",
            0.06,
            // 0.0,
            1,
            0,
            false
        ),
    ]
});

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

fn mixer(channels: &LazyLock<[Channel; N_CHAN]>) -> Vec<Level> {
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
            if let Some(midis) = channels[channel_idx].pattern.get_vec(&idx) {
                for midi in midis {
                    if midi.typ == START!() as NoteType {
                        synth_parameters.insert(
                            channel_idx * 128 + midi.note as usize,
                            SynthParameters::new(
                                FREQ_DATA[midi.note as usize],
                                channels[channel_idx].volume,
                                &channels[channel_idx].preset,
                                channels[channel_idx].n_poly,
                                channels[channel_idx].be_modulated,
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

fn generate_wav(name: &str, sample: Vec<Level>) {
    // 创建 WAV 文件规格
    let spec = hound::WavSpec {
        channels: 2, // 单声道
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 8, // 8位样本
        sample_format: hound::SampleFormat::Int,
    };

    
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("无法获取当前工作目录: {}", e);
            return;
        }
    };

    let wav_path = current_dir.join("wav");

    if wav_path.exists() {
    } else {
        match fs::create_dir(&wav_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("无法创建文件夹: {}", e);
            }
        }
    }

    let path = wav_path.join(format!("{}{}", name, ".wav"));
    let mut writer =
        hound::WavWriter::create(path, spec).unwrap();

    // 写入双声道音频
    for i in 0..sample.len() {
        writer
            .write_sample(((sample[i] as f32) * 0.65) as Level)
            .unwrap();
        if i < 50 {
            writer.write_sample(0).unwrap();
        } else {
            writer.write_sample(sample[i - 50]).unwrap();
        }
    }
    writer.finalize().unwrap();
}

fn load_wav(name: &str) {
    // 尝试打开音频文件
    // 20241021House Project(2024 Edit).wav");

    // 获取默认音频输出设备
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("无法获取默认输出设备: {}", e);
            return;
        }
    };

    // 创建音频播放器
    let sink = match Sink::try_new(&stream_handle) {
        Ok(sink) => sink,
        Err(e) => {
            eprintln!("无法创建音频播放器: {}", e);
            return;
        }
    };
    let mut i = 0;
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("无法获取当前工作目录: {}", e);
            return;
        }
    };
    let wav_path = current_dir.join("wav");
    // println!({},wav_path);
    let path = wav_path.join(format!("{}{}", name, ".wav"));
    while i < LOOP_TIMES {
        let file_result = File::open(&path);

        // 使用 match 处理 Result
        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                eprintln!("无法打开文件: {}", e);
                return;
            }
        };

        if i == 0 {
            thread::sleep(Duration::from_secs(1));
        }

        // 创建 BufReader 包装文件
        let source = match Decoder::new(BufReader::new(file)) {
            Ok(source) => source,
            Err(e) => {
                eprintln!("无法解码音频文件: {}", e);
                return;
            }
        };
        i += 1;
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn main() {
    let name = "my_wave";
    let sample = mixer(&CHANNELS);
    generate_wav(name, sample);
    load_wav(name);
}
