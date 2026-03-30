use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn search_in_file(path: &str, keyword: &str) {
    let file = File::open(path);

    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for (num, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if line.contains(keyword) {
                    println!("{}:{} -> {}", path, num + 1, line);
                }
            }
        }
    }
}
