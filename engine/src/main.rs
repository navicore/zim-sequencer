use std::io::{self, BufRead, Write};

fn analyze_line(line: &str) -> String {
    if line.contains("C") && line.contains("E") && line.contains("G") {
        "[✔] Detected C major triad".to_string()
    } else {
        format!("[ℹ] Parsed: {}", line)
    }
}

fn main() {
    println!("🎵 zim-sequencer engine started");
    let _ = io::stdout().flush();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(code) => {
                println!("⏩ Received: {}", code);
                let _ = io::stdout().flush();

                let response = analyze_line(&code);
                println!("{}", response);
                let _ = io::stdout().flush();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                let _ = io::stderr().flush();
            }
        }
    }
}
