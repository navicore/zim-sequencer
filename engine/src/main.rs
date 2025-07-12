use std::io::{self, BufRead, BufReader, Write};

fn analyze_line(line: &str) -> String {
    let notes: Vec<&str> = line.split_whitespace().collect();
    
    if notes.is_empty() {
        return String::new();
    }
    
    let mut output = String::new();
    output.push_str(&format!("═══ Analysis ═══\n"));
    output.push_str(&format!("Notes: {}\n", notes.join(" ")));
    
    // Basic chord detection
    if notes.len() >= 3 {
        if notes.contains(&"C") && notes.contains(&"E") && notes.contains(&"G") {
            output.push_str("Chord: C major\n");
            output.push_str("Quality: Major triad\n");
            output.push_str("Intervals: M3, P5\n");
        } else if notes.contains(&"A") && notes.contains(&"C") && notes.contains(&"E") {
            output.push_str("Chord: A minor\n");
            output.push_str("Quality: Minor triad\n");
            output.push_str("Intervals: m3, P5\n");
        } else {
            output.push_str(&format!("Chord: Unknown ({} notes)\n", notes.len()));
        }
    } else {
        output.push_str(&format!("Single notes or interval\n"));
    }
    
    output.push_str("═══════════════\n");
    output
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
        eprintln!("DEBUG: received: {}", trimmed); // also log to stderr
        println!("{}", analyze_line(trimmed));
        let _ = io::stdout().flush();
    }
}
