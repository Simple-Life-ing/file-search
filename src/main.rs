use std::env;
use walkdir::WalkDir;

mod search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir = &args[1];
    let keyword = &args[2];

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            let path = entry.path().display().to_string();
            search::search_in_file(&path, keyword);
        }
    }
}
