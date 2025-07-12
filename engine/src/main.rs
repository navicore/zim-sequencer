use std::io::{self, BufRead, BufReader, Write};

fn analyze_line(line: &str) -> String {
    if line.contains("C") && line.contains("E") && line.contains("G") {
        "[✔] Detected C major triad".to_string()
    } else {
        format!("[ℹ] Parsed: {}", line)
    }
}

fn main() {
    println!("🎵 zim-sequencer engine started");
    let _ = io::stdout().flush(); // Ensure initial message prints

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
