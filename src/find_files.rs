use std::ffi::OsStr;
use walkdir::{DirEntry, WalkDir};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use crate::filter_lines::create_line_regex;

pub fn find_all_files(path_dir: &str, with_extension: &str) -> Vec<DirEntry> {
    let mut list = Vec::new();

    for file in WalkDir::new(path_dir.clone()).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() && file.path().extension().and_then(OsStr::to_str) == Some(with_extension.clone()) {
            list.push(file.clone())
        }
    };
    return list.to_vec();
}

pub fn read_lines_of_files(files: Vec<DirEntry>, regex: &str) -> Vec<Vec<Vec<String>>> {
    let re = create_line_regex(regex.clone());
    let mut list = Vec::new();

    let files_first = files.clone();
    let files_second = files.clone();

    let mut inner_list: Vec<PathBuf> = Vec::new();

    for file_first in files_first.iter() {
        let module_path = get_module_path(&file_first).to_path_buf();

        if !inner_list.contains(&module_path) {
            inner_list.push(module_path);
        }
    }


    for module in inner_list.into_iter() {
        let mut module_list = Vec::new();

        for file in files_second.clone().into_iter() {
            if let Ok(lines) = read_lines(file.path()) {
                for line in lines {
                    let mut inner_vec = Vec::new();

                    let read_line = line.unwrap_or("no line read".parse().unwrap());
                    if re.is_match(&read_line) {
                        inner_vec.push(read_line.clone());
                        inner_vec.push(file.path().to_str().unwrap().to_string().clone());
                        inner_vec.push(module.to_str().unwrap().to_string().clone())
                    }

                    if get_module_path(&file) == module {
                        module_list.push(inner_vec)
                    }
                }
            }
        }
        list.push(module_list)
    }


    return list.clone();
}

fn get_module_path(file: &DirEntry) -> &Path {
    let position = file.path().into_iter().position(|dir| dir.to_ascii_lowercase().to_string_lossy().as_ref() == "src").unwrap();

    let mut path = file.path();
    for (i, _dir) in file.path().into_iter().enumerate() {
        if i < position {
            path = path.parent().unwrap();
        }
    }

    path
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}