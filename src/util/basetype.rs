use multimap::MultiMap;

pub type Level = i8;
pub type Timebase = u32;
pub type Timestamp = u32;
pub type FTimestamp = f32;
pub type NoteType = i8;
pub type Note = u8;

pub struct Midi {
    pub note: Note,
    pub typ: NoteType,
}

pub struct ModulateParameters {
    pub frequency: f32,
    pub range: f32,
}

pub type Score = MultiMap<Timebase, Midi>;