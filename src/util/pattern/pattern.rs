use std::fs::File;
use std::io::Write;

use crate::util::basetype::ChannelID;
use crate::util::basetype::Midi;
use crate::util::parameter::baseconst::NOTE_NUM;
use crate::util::parameter::baseconst::SONG_LEN;
use crate::Channel;
use crate::Note;
use crate::Score;
use crate::Timebase;
use crate::END;
use crate::START;

pub struct Pattern {
    name: String,
    score: Score,
    start_time: Timebase,
    len: Timebase,
    // channel_id: ChannelID,
}

impl Pattern {
    pub fn new(t: Timebase, name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: Score::new(),
            start_time: t,
            len: 0,
            // channel_id: channel_id,
        }
    }

    pub fn get_len(&self) -> Timebase {
        self.len
    }

    // pub fn get_channel_id(&self) -> ChannelID {
    //     self.channel_id
    // }

    pub fn get_start_time(&self) -> Timebase {
        self.start_time
    }

    pub fn get_score(&self) -> &Score {
        &self.score
    }

    pub fn get_vec(&self, t: Timebase) -> Option<&Vec<Midi>> {
        // t为音符在全曲的绝对时间，查找是需要减去pattern开始的时间，变成音符相对pattern的时间才能查询到
        if t < self.start_time {
            return None;
        }
        let relative_time = t - self.start_time;
        self.get_score().get_vec(&relative_time)
    }

    // pub fn set_channel_id(&mut self, new_id: ChannelID) {
    //     self.channel_id = new_id;
    // }

    pub fn set_start_time(&mut self, new_start_time: Timebase) {
        self.start_time = new_start_time;
    }

    pub fn copy_notes_from(&mut self, new_notes: &Score) {
        self.score.clone_from(new_notes);
        // 然后更新len
        self.update_len();
    }

    pub fn insert_note(
        &mut self,
        note_idx: Note,
        start_time: Timebase,
        end_time: Timebase,
    ) -> Result<(), &str> {
        self.change_note(note_idx, start_time, end_time, false)
    }

    pub fn delete_note(
        &mut self,
        note_idx: Note,
        start_time: Timebase,
        end_time: Timebase,
    ) -> Result<(), &str> {
        self.change_note(note_idx, start_time, end_time, true)
    }

    fn update_len(&mut self) {
        let mut max_len: Timebase = 0;
        for i in 0..(SONG_LEN * 16 + 1) {
            let _notes = match self.score.get_vec(&i) {
                Some(_x) => {
                    max_len = i;
                    continue;
                } // 如果这个时间有音符，查是否有相同的
                _ => {
                    continue;
                } // 如果这个时间没有音符，继续查询
            };
        } // for i
        self.len = max_len;
    }

    fn change_note(
        &mut self,
        idx: Note,
        st: Timebase,
        et: Timebase,
        delete: bool,
    ) -> Result<(), &str> {
        // 如果相同音符的end time在st et中间，删掉end time，插入et；
        // 如果相同音符的start time在st et中间，删掉start time，插入st；
        // 如果相同音符的start time和end time都在中间，删掉这两个，插入st和et；
        // 如果都不在，插入st和et。
        // 删除模式时，删掉st et中间的note操作保持一致，只是插入的st改为et，插入et改为st，且et、st对应start、end类型交换
        if et <= st {
            return Err("end time is smaller than start time!");
        }

        let mut insert_st: bool = false;
        let mut insert_et: bool = false;

        let mut deleted_et: Timebase = 0;

        for i in st..(et + 1) {
            let notes = match self.score.get_vec(&i) {
                Some(x) => x.clone(), // 如果这个时间有音符，查是否有相同的
                None => {
                    continue;
                } // 如果这个时间没有音符，继续查询
            }; // get notes
               // 遍历这个时间内的所有音符
            for j in notes {
                if j.note == idx {
                    // 如果插入范围内有相同音符，删除插入范围内的音符
                    let remain_vec = match self.score.remove(&i) {
                        Some(x) => x,
                        None => {
                            break;
                        }
                    }; // let remain_vec
                    for k in remain_vec {
                        if k.note != idx {
                            self.score.insert(i, k);
                        } // 插入那些不该删除的
                    } // for
                      // 检查范围内的音符的类型
                    match j.typ {
                        START!() => {
                            insert_st = true;
                        } // START
                        END!() => {
                            deleted_et = i;
                            insert_et = true;
                        } // END
                        _ => {}
                    } // match
                    break;
                } // if note == idx
            } // for j in notes
        } // for i in time range
          // 插入新音符
        if delete {
            // 检查是否在中间
            let mut is: bool = false; // st之前是否有note start
            let mut ie: bool = true; // et之后是否有note end
            for i in 0..(self.len + 1) {
                let notes = match self.score.get_vec(&i) {
                    Some(x) => x,
                    _ => {
                        continue;
                    } // 如果这个时间没有音符，继续查询
                };
                for j in notes {
                    if j.note == idx {
                        if i < st && j.typ == START!() {
                            is = true;
                        } else if i > et && j.typ == END!() {
                            ie = true;
                        }
                    } // if note == idx
                } // for j in notes
            } // for i in [0, len]
            let in_middle: bool = is && ie; // 对于删除，检查st、et在不在旧的st、et的中间
                                            // 如果删除的地方在中间，插入st作为前面start的end、插入et作为后面end的start
            if (insert_st && !insert_et) || in_middle {
                // 如果删除范围内有旧st且没有旧et，可以把参数et作为新的st；如果即有旧st又有旧et，全部删除，不应插入新的
                // 否则，因为删除旧的st，直接插入新的
                self.score.insert(
                    et,
                    Midi {
                        note: idx,
                        typ: START!(),
                    },
                );
            } // if insert et as start
            if (insert_et && !insert_st) || in_middle {
                // 如果删除范围内有旧et且没有旧st，可以把参数st作为新的et；如果即有旧st又有旧et，全部删除，不应插入新的
                // 否则，因为删除旧的et，直接插入新的
                self.score.insert(
                    st,
                    Midi {
                        note: idx,
                        typ: END!(),
                    },
                );
            } // if insert st as end
        }
        //if delete
        else {
            // 如果两个都是false，说明范围内没有相同音符，都插入
            if insert_st == insert_et {
                // 对于插入，如果范围内没有旧的st和et，说明插入新的音符，直接都插入
                // 对于删除，如果范围内没有旧的st和et，不需要做任何操作
                insert_st = true;
                insert_et = true;
            } // if ist == iet
            if insert_st {
                self.score.insert(
                    st,
                    Midi {
                        note: idx,
                        typ: START!(),
                    },
                );
            } // if insert_st
            if insert_et {
                self.score.insert(
                    et,
                    Midi {
                        note: idx,
                        typ: END!(),
                    },
                );
            } // if insert_et
        } // else (insert mode)

        // 更新pattern的时长
        if delete {
            // 如果旧的end time被删除，如果删除的et大于等于len，重新遍历map设置len
            if insert_et && (deleted_et >= self.len) {
                let ori_len = self.len;
                let mut max_len: Timebase = 0;
                // 如果删除，新的len一定小于等于当前的len
                // 遍历map从0到len，记录最大到哪个key存在对应的value
                for i in 0..(ori_len + 1) {
                    let _notes = match self.score.get_vec(&i) {
                        Some(_x) => {
                            max_len = i;
                            continue;
                        } // 如果这个时间有音符，查是否有相同的
                        _ => {
                            continue;
                        } // 如果这个时间没有音符，继续查询
                    };
                } // for i
                self.len = max_len;
                // 否则，不会影响len，因为此时idx对于的note最大时间戳没有减小
            } // if 删除了旧的et
        }
        // if delete
        else {
            self.len = match self.len.cmp(&et) {
                std::cmp::Ordering::Less => et,
                _ => self.len,
            };
        } // else
        return Ok(());
    }

    pub fn read_from_string(&mut self, lines: &Vec<&str>, line_idx: &mut usize) {
        // 接下来就是读notes了
        for i in 0..NOTE_NUM {
            // 先按空格parse
            let time_str: Vec<&str> = lines[*line_idx].trim().split(' ').collect();
            if time_str.is_empty() {
                continue;
            }
            // 读每行对应音符的开始结束时间
            let mut typ = START!();
            for s in time_str {
                if s.is_empty() {
                    break;
                }
                self.score
                    .insert(s.parse::<Timebase>().unwrap(), Midi { note: i, typ: typ as i8 });
                if typ == START!() {
                    typ = END!();
                } else {
                    typ = START!();
                } // if change typ
            } // for s
            // 读完一行就加一
            *line_idx += 1;
        } // for i
    } // fn read from string

    pub fn write_to_file(&self, file: &mut File) {
        // 格式为86行字符串，每行由偶数个数字构成，每对相邻数字为本行对应音符起止时间。
        // 先创造二维数组
        let mut pattern: Vec<Vec<Timebase>> = Vec::new();
        pattern.reserve(NOTE_NUM as usize);
        for _ in 0..NOTE_NUM {
            pattern.push(Vec::new());
        }

        // 遍历score，填充pattern
        for i in 0..(self.len + 1) {
            let notes = match self.score.get_vec(&i) {
                Some(x) => x, // 如果这个时间有音符，查是否有相同的
                None => {
                    continue;
                } // 如果这个时间没有音符，继续查询
            }; // get notes
               // 如果这个时刻有音符，插入这个音符的时间
            for j in notes {
                let idx = j.note as usize;
                pattern[idx].push(i);
            } // for j in notes
        } // for i in 0..len

        // 把pattern写入文件
        // 先写名字
        file.write(self.name.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        // 再写开始时间
        file.write(self.start_time.to_string().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        for i in 0..NOTE_NUM as usize {
            for j in &pattern[i] {
                // println!("{:?}", j.to_string().as_bytes());
                file.write(j.to_string().as_bytes()).unwrap();
                file.write(b" ").unwrap();
            } // for j in pattern[i]
            file.write(b"\n").unwrap();
        } // for i in 0..86
    } // fn pattern_file

    pub fn pattern_file(&self, file_path: &str) {
        let mut file = std::fs::File::create(file_path).expect("Create pattern file failed!");
        self.write_to_file(&mut file);
    }
}
