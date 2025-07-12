use std::io::{self, BufRead, BufReader, Write};

mod music_theory;
use music_theory::{Note, parse_note, analyze_chord};

fn parse_transformation(input: &str) -> (Vec<Note>, Option<String>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut notes = Vec::new();
    let mut transform = None;
    let mut i = 0;
    
    // Parse notes until we hit a transformation operator
    while i < parts.len() {
        if parts[i].starts_with('+') || parts[i].starts_with('-') || 
           parts[i].starts_with('*') || parts[i].starts_with('/') ||
           parts[i].ends_with('c') || parts[i].ends_with("ct") ||
           parts[i] == "inv" || parts[i] == "reverse" || parts[i] == "spread" ||
           parts[i] == "just" || parts[i].starts_with("edo") {
            transform = Some(parts[i..].join(" "));
            break;
        }
        
        if let Some(note) = parse_note(parts[i]) {
            notes.push(note);
        }
        i += 1;
    }
    
    (notes, transform)
}

fn apply_transformation(notes: &[Note], transform: &str) -> Vec<Note> {
    let parts: Vec<&str> = transform.split_whitespace().collect();
    if parts.is_empty() {
        return notes.to_vec();
    }
    
    match parts[0] {
        // Transposition: +n or -n
        s if s.starts_with('+') => {
            if let Ok(semitones) = s[1..].parse::<i32>() {
                notes.iter().map(|n| n.transpose(semitones)).collect()
            } else {
                notes.to_vec()
            }
        },
        s if s.starts_with('-') => {
            if let Ok(semitones) = s[1..].parse::<i32>() {
                notes.iter().map(|n| n.transpose(-semitones)).collect()
            } else {
                notes.to_vec()
            }
        },
        // Frequency multiplication
        s if s.starts_with('*') => {
            if let Ok(ratio) = s[1..].parse::<f32>() {
                notes.iter().map(|n| n.multiply_freq(ratio)).collect()
            } else {
                notes.to_vec()
            }
        },
        // Cents adjustment
        s if s.ends_with('c') || s.ends_with("ct") => {
            let cents_str = s.trim_end_matches("ct").trim_end_matches('c');
            if let Ok(cents) = cents_str.parse::<f32>() {
                notes.iter().map(|n| n.transpose_cents(cents)).collect()
            } else {
                notes.to_vec()
            }
        },
        // Inversion
        "inv" => {
            if notes.is_empty() {
                return notes.to_vec();
            }
            let pivot = notes[0];
            notes.iter().map(|n| {
                let freq_ratio = n.to_frequency() / pivot.to_frequency();
                Note::from_frequency(pivot.to_frequency() / freq_ratio)
            }).collect()
        },
        // Reverse order
        "reverse" => {
            let mut reversed = notes.to_vec();
            reversed.reverse();
            reversed
        },
        // Spread voices
        "spread" => {
            notes.iter().enumerate().map(|(i, n)| {
                n.transpose(i as i32 * 12)
            }).collect()
        },
        // Just intonation ratios
        "just" => {
            if notes.is_empty() {
                return notes.to_vec();
            }
            let root = notes[0];
            vec![
                root,                              // 1/1
                root.multiply_freq(9.0/8.0),      // Major 2nd
                root.multiply_freq(5.0/4.0),      // Major 3rd  
                root.multiply_freq(4.0/3.0),      // Perfect 4th
                root.multiply_freq(3.0/2.0),      // Perfect 5th
                root.multiply_freq(5.0/3.0),      // Major 6th
                root.multiply_freq(15.0/8.0),     // Major 7th
            ]
        },
        // Equal temperament divisions
        s if s.starts_with("edo") => {
            if let Ok(divisions) = s[3..].parse::<u32>() {
                if notes.is_empty() || divisions == 0 {
                    return notes.to_vec();
                }
                let root = notes[0];
                (0..divisions).map(|i| {
                    root.multiply_freq(2.0_f32.powf(i as f32 / divisions as f32))
                }).collect()
            } else {
                notes.to_vec()
            }
        },
        _ => notes.to_vec()
    }
}

fn analyze_line(line: &str) -> String {
    let (notes, transform) = parse_transformation(line);
    
    if notes.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    output.push_str("═══ Analysis ═══\n");
    
    // Show original notes
    output.push_str(&format!("Input: {}\n", 
        notes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")));
    
    // Apply transformation if present
    let analyzed_notes = if let Some(ref t) = transform {
        let transformed = apply_transformation(&notes, t);
        output.push_str(&format!("Transform: {}\n", t));
        output.push_str(&format!("Result: {}\n", 
            transformed.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")));
        
        // Show frequencies for microtonal notes
        let has_microtones = transformed.iter().any(|n| n.cents.abs() > 0.1);
        if has_microtones {
            output.push_str("\nFrequencies (Hz):\n");
            for note in &transformed {
                output.push_str(&format!("  {}: {:.2} Hz\n", note, note.to_frequency()));
            }
        }
        
        transformed
    } else {
        notes.clone()
    };
    
    // Analyze the chord/intervals
    output.push_str(&analyze_chord(&analyzed_notes));
    
    // Add chord identification
    if analyzed_notes.len() >= 3 {
        output.push_str(&identify_chord_type(&analyzed_notes));
    }
    
    output.push_str("═══════════════\n");
    output
}

fn identify_chord_type(notes: &[Note]) -> String {
    let mut sorted_notes = notes.to_vec();
    sorted_notes.sort_by_key(|n| n.to_midi());
    
    if sorted_notes.len() < 3 {
        return String::new();
    }
    
    // Get intervals from root
    let root = &sorted_notes[0];
    let intervals: Vec<i32> = sorted_notes[1..]
        .iter()
        .map(|n| (root.interval_to(n) % 12) as i32)
        .collect();
    
    let chord_type = match intervals.as_slice() {
        [4, 7] => "Major triad",
        [3, 7] => "Minor triad", 
        [4, 7, 11] => "Major 7th",
        [3, 7, 10] => "Minor 7th",
        [4, 7, 10] => "Dominant 7th",
        [3, 6] => "Diminished triad",
        [4, 8] => "Augmented triad",
        [2, 7] => "Sus2",
        [5, 7] => "Sus4",
        _ => "Unknown chord type"
    };
    
    format!("\nChord type: {}\n", chord_type)
}

fn main() {
    // Start silently - the UI will show when ready

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();

    loop {
        input.clear();
        let bytes = reader.read_line(&mut input).unwrap();
        if bytes == 0 {
            continue; // skip empty
        }

        let trimmed = input.trim_end();
        println!("{}", analyze_line(trimmed));
        let _ = io::stdout().flush();
    }
}