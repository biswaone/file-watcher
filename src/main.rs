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

    let mut present_files: HashSet<PathBuf> = HashSet::new();
    let mut new_files: HashSet<PathBuf> = HashSet::new();
    // Recursively traverse directories
    traverse_directory(&path,&mut present_files);
    thread::sleep(Duration::from_secs(15));
    traverse_directory(&path,&mut new_files);

    let diff1: HashSet<&PathBuf> = present_files.difference(&new_files).collect();
    let diff2: HashSet<&PathBuf> = new_files.difference(&present_files).collect();

    for file2 in diff2 {
        println!("++{}", file2.display());
    }
    for file1 in diff1 {
        println!("--{}", file1.display());
    }
}

fn traverse_directory(path: &std::path::Path, files: &mut HashSet<PathBuf>){
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.insert(entry.path().to_path_buf());
        }
    }
}



