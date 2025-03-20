use crate::Channel;
use crate::Pattern;
use crate::Score;
use std::string;
use std::vec;

use super::basetype::Timebase;
// use std::sync::LazyLock;

pub struct Song {
    pub patterns: Vec<Vec<Pattern>>,
    pub channels: Vec<Channel>,
    pub name: String,
} // struct Song

impl Song {
    pub fn new(name: &str) -> Self {
        Song {
            patterns: Vec::new(),
            channels: Vec::new(),
            name: name.to_string(),
        } // Song
    } // fn new

    pub fn new_channel(
        &mut self,
        name: &str,
        preset: &str,
        volume: f32,
        n_poly: usize,
        pan: i8,
        be_modulated: bool,
    ) {
        self.channels.push(Channel {
            name: name.to_string(),
            preset: preset.to_string(),
            volume: volume,
            n_poly: n_poly,
            pan: pan,
            be_modulated: be_modulated,
        });
        self.patterns.push(Vec::new());
    }

    pub fn new_pattern(&mut self, channel_id: usize, start_time: Timebase) -> Result<(), &str> {
        if channel_id >= self.channels.len() {
            return Err("Channel index out of boundary!");
        } // if out of boundary

        self.patterns[channel_id].push(Pattern::new(start_time));
        self.sort_patterns(channel_id);
        Ok(())
    }

    pub fn move_pattern_time(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        new_start_time: Timebase,
    ) -> Result<(), &str> {
        if channel_id >= self.channels.len() || pattern_id >= self.patterns[channel_id].len() {
            return Err("Channel index or Pattern index out of boundary!");
        }

        self.patterns[channel_id][pattern_id].set_start_time(new_start_time);
        self.sort_patterns(channel_id);

        Ok(())
    }

    pub fn move_pattern_channel(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        new_channel_id: usize,
    ) -> Result<(), &str> {
        if channel_id >= self.channels.len()
            || new_channel_id >= self.channels.len()
            || pattern_id >= self.patterns[channel_id].len()
        {
            return Err("Channel index or Pattern index out of boundary!");
        }

        let tmp_pattern = self.patterns[channel_id].remove(pattern_id);
        self.patterns[new_channel_id].push(tmp_pattern);
        self.sort_patterns(channel_id);
        self.sort_patterns(new_channel_id);

        Ok(())
    }

    pub fn copy_pattern_from(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        score: &Score,
    ) -> Result<(), &str> {
        if channel_id >= self.channels.len() || pattern_id >= self.patterns[channel_id].len() {
            return Err("Channel index or Pattern index out of boundary!");
        }

        self.patterns[channel_id][pattern_id].copy_notes_from(score);

        Ok(())
    }

    pub fn edit_pattern(
        &mut self,
        channel_id: usize,
        pattern_id: usize,
        mode: &str,
        note_idx: u8,
        start_time: Timebase,
        end_time: Timebase,
    ) -> Result<(), &str> {
        if channel_id >= self.channels.len() || pattern_id >= self.patterns[channel_id].len() {
            return Err("Channel index or Pattern index out of boundary!");
        }

        let mode = mode.to_string();
        if mode == "delete" {
            return self.patterns[channel_id][pattern_id]
                .delete_note(note_idx, start_time, end_time);
        } else if mode == "insert" {
            return self.patterns[channel_id][pattern_id]
                .insert_note(note_idx, start_time, end_time);
        }

        Err("Wrong mode!")
    }

    fn sort_patterns(&mut self, channel_id: usize) {
        // 按照start time对一个channel的pattern排序
        self.patterns[channel_id].sort_by_key(|a| a.get_start_time());
    } // fn sort_patterns
} // impl Song

/*
pub static SONG: LazyLock<[Channel; N_CHAN]> = LazyLock::new(|| {
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
*/
