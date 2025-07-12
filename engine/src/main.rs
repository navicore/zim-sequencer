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
        match reader.read_line(&mut input) {
            Ok(0) => continue,
            Ok(_) => {
                println!("⏩ Received: {}", input.trim_end());
                let response = analyze_line(&input);
                println!("{}", response);
                let _ = io::stdout().flush();
            }
            Err(e) => {
                eprintln!("error: {}", e);
                let _ = io::stderr().flush();
            }
        }
    }
}
