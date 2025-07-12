use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub pitch_class: u8,  // 0-11 (C=0, C#=1, D=2, etc.)
    pub octave: i8,       // typically -2 to 8
    pub cents: f32,       // -50 to +50 cents deviation
}

impl Note {
    pub fn new(pitch_class: u8, octave: i8) -> Self {
        Note {
            pitch_class: pitch_class % 12,
            octave,
            cents: 0.0,
        }
    }

    pub fn new_with_cents(pitch_class: u8, octave: i8, cents: f32) -> Self {
        Note {
            pitch_class: pitch_class % 12,
            octave,
            cents: cents.clamp(-50.0, 50.0),
        }
    }

    pub fn from_midi(midi: u8) -> Self {
        Note {
            pitch_class: midi % 12,
            octave: (midi / 12) as i8 - 1,
            cents: 0.0,
        }
    }

    pub fn from_frequency(freq: f32) -> Self {
        // A4 = 440Hz, MIDI note 69
        let midi_float = 69.0 + 12.0 * (freq / 440.0).log2();
        let midi_note = midi_float.floor() as u8;
        let cents = (midi_float - midi_note as f32) * 100.0;
        
        let mut note = Note::from_midi(midi_note);
        note.cents = cents;
        note
    }

    pub fn to_frequency(&self) -> f32 {
        let midi_float = self.to_midi() as f32 + (self.cents / 100.0);
        440.0 * 2.0_f32.powf((midi_float - 69.0) / 12.0)
    }

    pub fn to_midi(&self) -> u8 {
        ((self.octave + 1) * 12 + self.pitch_class as i8) as u8
    }

    pub fn transpose(&self, semitones: i32) -> Self {
        let midi = self.to_midi() as i32 + semitones;
        let mut note = Note::from_midi(midi.clamp(0, 127) as u8);
        note.cents = self.cents;
        note
    }

    pub fn transpose_cents(&self, cents: f32) -> Self {
        let total_cents = self.cents + cents;
        let extra_semitones = (total_cents / 100.0).floor() as i32;
        let remaining_cents = total_cents - (extra_semitones as f32 * 100.0);
        
        let mut note = self.transpose(extra_semitones);
        note.cents = remaining_cents.clamp(-50.0, 50.0);
        note
    }

    pub fn multiply_freq(&self, ratio: f32) -> Self {
        let freq = self.to_frequency();
        Note::from_frequency(freq * ratio)
    }

    pub fn interval_to(&self, other: &Note) -> i32 {
        other.to_midi() as i32 - self.to_midi() as i32
    }

    pub fn name(&self) -> &'static str {
        match self.pitch_class {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cents.abs() < 0.1 {
            write!(f, "{}{}", self.name(), self.octave)
        } else {
            write!(f, "{}{}({:+.0}¢)", self.name(), self.octave, self.cents)
        }
    }
}

pub fn parse_note(s: &str) -> Option<Note> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Check for cents notation (e.g., "C4+25" or "E5-14")
    let (base_note, cents) = if let Some(pos) = s.find('+') {
        let (base, cents_str) = s.split_at(pos);
        (base, cents_str[1..].parse::<f32>().ok())
    } else if let Some(pos) = s.rfind('-') {
        // Use rfind to handle negative octaves
        let (base, cents_str) = s.split_at(pos);
        if base.ends_with(char::is_numeric) && cents_str[1..].chars().all(|c| c.is_numeric() || c == '.') {
            (base, cents_str.parse::<f32>().ok())
        } else {
            (s, None)
        }
    } else {
        (s, None)
    };

    let (note_part, octave_part) = if base_note.ends_with(char::is_numeric) {
        let split_pos = base_note.rfind(|c: char| !c.is_numeric()).map(|i| i + 1).unwrap_or(0);
        base_note.split_at(split_pos)
    } else {
        (base_note, "4")  // default octave
    };

    let pitch_class = match note_part.to_uppercase().as_str() {
        "C" => 0,
        "C#" | "CS" => 1,
        "D" => 2,
        "D#" | "DS" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "FS" => 6,
        "G" => 7,
        "G#" | "GS" => 8,
        "A" => 9,
        "A#" | "AS" => 10,
        "B" => 11,
        _ => return None,
    };

    let octave = octave_part.parse::<i8>().ok()?;
    match cents {
        Some(c) => Some(Note::new_with_cents(pitch_class, octave, c)),
        None => Some(Note::new(pitch_class, octave)),
    }
}

pub fn interval_name(semitones: i32) -> &'static str {
    match semitones.abs() % 12 {
        0 => "P1 (unison)",
        1 => "m2 (minor 2nd)",
        2 => "M2 (major 2nd)",
        3 => "m3 (minor 3rd)",
        4 => "M3 (major 3rd)",
        5 => "P4 (perfect 4th)",
        6 => "TT (tritone)",
        7 => "P5 (perfect 5th)",
        8 => "m6 (minor 6th)",
        9 => "M6 (major 6th)",
        10 => "m7 (minor 7th)",
        11 => "M7 (major 7th)",
        _ => unreachable!(),
    }
}

pub fn analyze_chord(notes: &[Note]) -> String {
    if notes.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    
    // Sort notes by pitch
    let mut sorted_notes = notes.to_vec();
    sorted_notes.sort_by_key(|n| n.to_midi());
    
    // Analyze intervals from root
    if sorted_notes.len() > 1 {
        output.push_str("Intervals from root:\n");
        let root = &sorted_notes[0];
        for note in &sorted_notes[1..] {
            let interval = root.interval_to(note);
            output.push_str(&format!("  {} → {}: {} semitones ({})\n", 
                root, note, interval, interval_name(interval)));
        }
    }
    
    // Analyze consecutive intervals
    if sorted_notes.len() > 2 {
        output.push_str("\nConsecutive intervals:\n");
        for i in 0..sorted_notes.len() - 1 {
            let interval = sorted_notes[i].interval_to(&sorted_notes[i + 1]);
            output.push_str(&format!("  {} → {}: {} semitones\n",
                sorted_notes[i], sorted_notes[i + 1], interval));
        }
    }
    
    output
}