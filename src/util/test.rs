use crate::Pattern;
use crate::Channel;
use crate::N_CHAN;
use crate::midi_generator;

pub fn test_pattern() {
    let mut p = Pattern::new(0);
    p.insert_note(0, 2, 4).unwrap();
    p.insert_note(0, 3, 8).unwrap();
    p.insert_note(0, 12, 23).unwrap();
    p.insert_note(0, 10, 14).unwrap();
    p.insert_note(1, 5, 123).unwrap();
    for i in 2..86 {
        let st = (i as u32) / 2;
        p.insert_note(i, st, st + st / 2 + 1).unwrap();
    }
    p.delete_note(0, 5, 20).unwrap();
    p.delete_note(1, 0, 130).unwrap();
    p.delete_note(2, 1, 2).unwrap();
    p.delete_note(82, 45, 46).unwrap();

    println!("{}", p.get_len());

    p.pattern_file("pt.txt");
} // fn test_pattern

pub fn init_test_pattern() -> Vec<Vec<Pattern>>{
    let mut patterns: Vec<Vec<Pattern>> = Vec::new();
    for i in 0..N_CHAN {
        patterns.push(Vec::new());
        patterns[i].push(Pattern::new(0));
    }
    patterns[0][0].copy_notes_from(&midi_generator("
        --------|------C4(-),D4(-),||

        D#4(-----),G4(-),F4(-),D#4(-),|F4(-),D4(-----),A#3(-),C4(-),|
        D4(-----),F4(-),D#4(-),D4(-),|D#4(-),C4(-----),D#4(-),F4(-),|
        G4(-----),A#4(-),G#4(-),G4(-),|G#4(-),F4(-----),C4(-),D4(-),|
        D#4(-----),F4(-),D#4(-),C4(-),|D4(------),C4(-),D4(-),||"));
    let tmp = patterns[0][0].get_len();
    patterns[0].push(Pattern::new(tmp));
    patterns[0][1].copy_notes_from(&midi_generator("
        D#4(-----),G4(-),F4(-),D#4(-),|F4(-),D4(-----),A#3(-),C4(-),|
        D4(-----),F4(-),D#4(-),D4(-),|D#4(-),C4(-----),D#4(-),F4(-),|
        G4(-----),A#4(-),G#4(-),G4(-),|G#4(-),F4(-----),C4(-),D4(-),|
        D#4(-----),F4(-),D#4(-),C4(-),|G4(------),F4(-),G4(-),||"));
    let tmp =tmp + patterns[0][1].get_len();
    patterns[0].push(Pattern::new(tmp));
    patterns[0][2].copy_notes_from(&midi_generator("
        G#4(------),D4(-),D#4(-)|F4(------),C4(-),D4(-)|
        D#4(------),C4(-),G3(-)|C4(-)D4(-)C4(-)D4(-)D#4(--)G4(-)G#4(-)||
        C5(-----)G#4(--)A#4(-)|D5(-----)G#4(--)A#4(-)|
        C4(=)D#4(=)F#4(=)A4(=)C5(=)D#5(=)F#5(=)A5(=)C6(=)A5(=)F#5(=)D#5(=)C5(=)A4(=)F#4(=)D#4(=)|D4(--------)||B3(--------)||"));
    
    patterns[1][0].copy_notes_from(&midi_generator("
        C6D#6-(-)-(-)-(-)-(-)|-(-)-(-)-(-)-(-)||
        C6D#6-(-)-(-)-(-)-(-)|A#5D6-(-)-(-)-(-)-(-)|
         -(-)-(-)-(-)-(-)|G5C6-(-)-(-)-(-)-(-)|
        A#5D#6-(-)-(-)-(-)-(-),|A#5D6-(-)-(-)-(-)-(-)|
        G5C6-(-)-(-)-(-)-(-)|G5B5-(-)-(-)-(-)-(-)||"));
    let tmp = patterns[1][0].get_len();
    patterns[1].push(Pattern::new(tmp));
    patterns[1][1].copy_notes_from(&midi_generator("C6D#6-(-)-(-)-(-)-(-)|A#5D6-(-)-(-)-(-)-(-)|
         -(-)-(-)-(-)-(-)|G5C6-(-)-(-)-(-)-(-)|
        A#5D#6-(-)-(-)-(-)-(-),|A#5D6-(-)-(-)-(-)-(-)|
        G5C6-(-)-(-)-(-)-(-)|G5B5-(-)-(-)-(-)-(-)||
        A#5D6-(-)-(-)----|G5A#5-(-)-(-)----|
        G#5C6-(-)-(-)----|D#5G5-(-)-(-)----|
        F5G#5C6-(-)-(=)=(--)--|G5A#5D6-(-)-(=)=(--)--||
        A#5D6G6(--)-----G5B5D6(=)=|(-)-B5D6F6(=)=G5B5D6(-----)||F5G#5B5(=)=G#5B5F5(-)-F5G#5B5(-----)||"));
    
    patterns[2][0].copy_notes_from(&midi_generator("
        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),||

        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),||"));
    let tmp = patterns[2][0].get_len();
    patterns[2].push(Pattern::new(tmp));
    patterns[2][1].copy_notes_from(&midi_generator("D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||

        C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),C3(-),C4(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),||"));
    let tmp = tmp + patterns[2][1].get_len();
    patterns[2].push(Pattern::new(tmp));
    patterns[2][2].copy_notes_from(&midi_generator("D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),D#3(-),D#4(-),
        A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),A#2(-),A#3(-),
        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||

        G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),G#2(-),G#3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),
        F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-)||"));
    let tmp = tmp + patterns[2][2].get_len();
    patterns[2].push(Pattern::new(tmp));
    patterns[2][3].copy_notes_from(&midi_generator("F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),F2(-),F3(-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),|
        G#2(---=)G3(=-)A#3(-)G#3(=)D3(=-),
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||
        G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),G2(-),G3(-),||"));
    
    patterns[3][0].copy_notes_from(&midi_generator("
        --------|--------||
        --------|-F5(-),D#5(=),D5(=),D#5(-),F5(-),---|--------|-D#5(-),D5(=),C5(=),D5(-),D#5(-),---|
        --------|-G#5(-),G5(=),F5(=),G5(-),G#5(-),---|--------|-G5(=),F5(=),D#5(-),F5(-),D5(-),---||
        --------|-F5(-),D#5(=),D5(=),D#5(-),F5(-),---|--------|-D#5(-),D5(=),C5(=),D5(-),D#5(-),---|
        --------|-G#5(-),G5(=),F5(=),G5(-),G#5(-),---|--------|-F5(=),D#5(=),D5(-),D#5(-),C5(--),B4(--)||
        --D5(-)C5(-)F5(--)--|--C5(-)B4(-)D5(--)--|--G#4(-)G4(-)D5(-)B4(=)C5(=)G4(--)|G5(-),G#5(-),G5(-),F#5(-),G5(--)--|
        --------|--------|
        C4(=)D#4(=)F#4(=)A4(=)C5(=)D#5(=)F#5(=)A5(=)C6(=)A5(=)F#5(=)D#5(=)C5(=)A4(=)F#4(=)D#4(=)|

            "));
    
    patterns[4][0].copy_notes_from(&midi_generator("C2
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
            "));
    patterns
}

pub fn init_test_channel() -> Vec<Channel>{
    let mut channels: Vec<Channel> = Vec::new();
    channels.push(Channel { name: "1".to_string(), preset: "saw".to_string(), volume: 0.065, n_poly: 1, pan: 0, be_modulated: true });
    channels.push(Channel { name: "2".to_string(), preset: "square".to_string(), volume: 0.05, n_poly: 1, pan: 0, be_modulated: true });
    channels.push(Channel { name: "3".to_string(), preset: "triangle".to_string(), volume: 0.6, n_poly: 1, pan: 0, be_modulated: false });
    channels.push(Channel { name: "4".to_string(), preset: "spike".to_string(), volume: 0.1, n_poly: 1, pan: 0, be_modulated: true });
    channels.push(Channel { name: "5".to_string(), preset: "noise".to_string(), volume: 0.06, n_poly: 1, pan: 0, be_modulated: false });
    channels
}