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
    let _ = io::stdout().flush();

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input).unwrap();
        if bytes_read == 0 {
            continue; // EOF or nothing
        }

        println!("⏩ Received: {}", input.trim_end());
        let response = analyze_line(&input);
        println!("{}", response);
        let _ = io::stdout().flush();
    }
}
