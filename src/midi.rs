use std::io::{Read, Error, ErrorKind};

#[derive(Debug)]
pub struct MidiHeader {
    format_type: u16,
    num_tracks: u16,
    time_division: u16,
}

#[derive(Debug)]
pub enum MidiError {
    InvalidHeader,
    IOError(std::io::Error),
    UnexpectedEOF,
}

impl From<std::io::Error> for MidiError {
    fn from(err: std::io::Error) -> Self {
        MidiError::IOError(err)
    }
}

pub fn parse_header<R: Read>(reader: &mut R) -> Result<MidiHeader, MidiError> {
    let mut buffer = [0u8; 14];
    reader.read_exact(&mut buffer)?;

    if &buffer[0..4] != b"MThd" {
        return Err(MidiError::InvalidHeader);
    }

    let format_type = u16::from_be_bytes([buffer[9], buffer[10]]);
    let num_tracks = u16::from_be_bytes([buffer[11], buffer[12]]);
    let time_division = u16::from_be_bytes([buffer[13], buffer[14]]);

    Ok(MidiHeader {
        format_type,
        num_tracks,
        time_division,
    })
}

pub fn parse_track<R: Read>(reader: &mut R) -> Result<Vec<MidiEvent>, MidiError> {
    // 轨道解析实现占位
    unimplemented!()
}

#[derive(Debug)]
pub enum MidiEvent {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
    // 其他事件类型占位
}

fn read_variable_length<R: Read>(reader: &mut R) -> Result<u32, MidiError> {
    let mut value = 0u32;
    for _ in 0..4 {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        value = (value << 7) | (byte[0] as u32 & 0x7F);
        if (byte[0] & 0x80) == 0 {
            break;
        }
    }
    Ok(value)
}
