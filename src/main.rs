use std::env;
use std::thread;
use walkdir::WalkDir;

mod search;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <directory> <keyword>", args[0]);
        std::process::exit(1);
    }

    let dir = &args[1];
    let keyword = &args[2];

    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let chunk_size = (entries.len() + num_threads - 1) / num_threads;

    thread::scope(|s| {
        for chunk in entries.chunks(chunk_size) {
            s.spawn(move || {
                for entry in chunk {
                    if entry.file_type().is_file() {
                        let path = entry.path().display().to_string();
                        search::search_in_file(&path, keyword);
                    }
                }
            });
        }
    });
}
