use std::env::current_dir;
use std::fmt::Error;
use std::fs;
use std::path::PathBuf;

pub fn find_dir(asked_dir_path: String) -> PathBuf {
    for entry in fs::read_dir(asked_dir_path).unwrap() {
        let path = entry.unwrap().path();
        println!("content of {} {} {}", asked_dir_path.clone() , ": " , path.file_name().unwrap().to_str().unwrap());
    }

    let path_entry = fs::read_dir(asked_dir_path.clone());
    let path_buf = path_entry.unwrap().last().unwrap().unwrap().path();

    return path_buf

}

