use core::option::Option;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    module_path: String,
    lines: Vec<String>,
}




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

pub fn read_lines_of_files(files: Vec<DirEntry>, regex: &str) -> Vec<Module> {
    let re = create_line_regex(regex.clone());
    let mut list: Vec<Module> = Vec::new();

    let files_first = files.clone();

    let mut inner_list: Vec<PathBuf> = Vec::new();

    for file_first in files_first.iter() {
        let module_path = get_module_path(&file_first).or(Some(&Path::new(""))).unwrap().to_path_buf();

        if !inner_list.contains(&module_path) && module_path.ends_with("src") {
            inner_list.push(module_path);
        }
    }

    let modules = inner_list.into_iter().collect::<HashSet<_>>();

    for module in modules.into_iter() {
        let mut inner_vec = Vec::new();

        for file in WalkDir::new(module.clone()).into_iter() {
            let file_copy = file.unwrap().clone().into_path();


            if let Ok(lines) = read_lines(file_copy.clone()) {
                for line in lines {
                    let read_line = line.unwrap_or("no line read".parse().unwrap());
                    if re.is_match(&read_line) {
                        inner_vec.push(read_line.clone());
                    }
                }
            }
        }


        list.push(build_module(module.to_str().unwrap().to_string().clone(), inner_vec.clone()))
    }


    return list;
}

fn get_module_path(file: &DirEntry) -> Option<&Path> {
    let position = file.path().into_iter().position(|dir| dir.to_ascii_lowercase().to_string_lossy().as_ref() == "src");

    let mut path = Some(file.path());

    if  position.is_some(){
        for (i, _dir) in file.path().into_iter().enumerate() {
            if i < position.unwrap() {
                path = path.unwrap().parent();
            }
        }
        path
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn build_module(module: String, lines: Vec<String>) -> Module {
    Module {
        module_path: module.clone(),
        lines,
    }
}