use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::{thread, time::Duration};

use walkdir::WalkDir;


fn main() {
    let args: Vec<String> = env::args().collect();

    let path  = if args.len() > 1 {
        Path::new(&args[1])
    }else{
        Path::new(".")
    };

    loop {
        let present_files = traverse_directory(&path);
        // Every 15 sec go through the directory to see if any changes
        thread::sleep(Duration::from_secs(15));
        let new_files = traverse_directory(&path);

        let diff: HashSet<&PathBuf> = present_files.symmetric_difference(&new_files).collect();

        for file in diff {
            println!("{}", file.display());
        }
    }
}

fn traverse_directory(path: &std::path::Path) -> HashSet<PathBuf>{
    let mut files = HashSet::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.insert(entry.path().to_path_buf());
        }
    }
    files
}



