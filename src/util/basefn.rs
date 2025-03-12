use std::fs::File;
use std::io::BufReader;
use hound;
use rodio::{Decoder, OutputStream, Sink}; //, source::Source};
use std::thread;
use std::time::Duration;
use std::env;
use std::fs;

use crate::Score;
use crate::Timebase;
use crate::Note;
use crate::Level;
use crate::Midi;
use crate::START;
use crate::END;
use crate::SAMPLE_RATE;
use crate::LOOP_TIMES;

pub fn midi_generator(note: &str) -> Score {
    let mut tbase: Timebase = 0;
    let mut idx_vec: Vec<Note> = Vec::new();
    let mut score: Score = Score::new();
    let mut idx_tmp: Note = 0;
    let mut need_reset = false;
    for byte in note.bytes() {
        match byte {
            b'(' => {
                need_reset = true;
                for idx in &idx_vec{
                    score.insert(
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
                    score.insert(
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
    score
}

pub fn generate_wav(name: &str, sample: Vec<Level>) {
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

pub fn load_wav(name: &str) {
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