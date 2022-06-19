use std::ffi::OsStr;
use walkdir::{DirEntry, WalkDir};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn find_all_files(path_dir: &str, with_extension: &str) -> Vec<DirEntry> {
    let mut list = Vec::new();

    for file in WalkDir::new(path_dir).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() && file.path().extension().and_then(OsStr::to_str) == Some(with_extension) {
            list.push(file)
        }
    };

    println!("{:#?}", list);
    return list.to_vec();
}

pub fn read_lines_of_files(files: Vec<DirEntry>) -> Vec<String> {
    let mut list = Vec::new();

    for file in files.into_iter() {
        if let Ok(lines) = read_lines(file.path()) {
            for line in lines {
                list.push(line.unwrap_or("no line read".parse().unwrap()))
            }
        }
    }


    return list.to_vec();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}