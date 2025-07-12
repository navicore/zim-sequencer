use std::io::{BufRead, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut reader = std::io::BufReader::new(stdin);
    let mut input = String::new();

    loop {
        input.clear();
        if reader.read_line(&mut input).unwrap() == 0 {
            continue;
        }

        let trimmed = input.trim_end();
        let response = if trimmed.contains("C") && trimmed.contains("E") && trimmed.contains("G") {
            "[✔] Detected C major triad"
        } else {
            "[ℹ] Parsed input"
        };

        println!("⏩ Received: {}", trimmed);
        println!("{}", response);
        let _ = std::io::stdout().flush();
    }
}
