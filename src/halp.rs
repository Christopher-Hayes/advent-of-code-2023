use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(file_name: &str) -> String {
    let file = File::open(format!("input/{}", file_name)).expect("File not found");
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let s = format!("{}\n", line);
            output.push_str(&s);
        }
    }

    output
}
