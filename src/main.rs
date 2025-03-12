extern crate libm;
extern crate rand;

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
use util::synth::synthparameters::SynthParameters;
use util::song::SONG;

use util::synth::wavefn::multi_generator;

use util::effect::effectfn::fm_modulate;

use util::synth::synth;

use util::basefn::mixer;

use util::parameter::baseconst::N_TBASE;
use util::parameter::baseconst::SONG_LEN;
use util::parameter::baseconst::SAMPLE_RATE; // 采样率
use util::parameter::baseconst::BPM;
use util::parameter::baseconst::T_BEAT;
use util::parameter::baseconst::T_BASE;
use util::parameter::baseconst::N_CHAN;
// use util::parameter::baseconst::LOOP_TIMES;
use util::parameter::baseconst::LOOP_TIMES;
use util::parameter::baseconst::MAX_POLY;
use util::parameter::baseconst::FREQ_DATA;

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

fn main() {
    let name = "my_wave";
    let sample = mixer(&SONG);
    generate_wav(name, sample);
    load_wav(name);
}
