//! src/main.rs

use std::io::{self, BufRead};

fn analyze_line(line: &str) -> String {
    // Extremely basic harmony recognition
    if line.contains("C") && line.contains("E") && line.contains("G") {
        "[✔] Detected C major triad".to_string()
    } else if line.contains("D") && line.contains("F") && line.contains("A") {
        "[✔] Detected D minor triad".to_string()
    } else {
        format!("[ℹ] Parsed: {}", line)
    }
}

fn main() {
    println!("🎵 Text Sequencer REPL ready. Send code blocks via stdin.");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(code) => {
                let response = analyze_line(&code);
                println!("{}", response);
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}
