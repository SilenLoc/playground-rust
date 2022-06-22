use walkdir::DirEntry;
use crate::find_files::{read_lines_of_files};

pub fn get_matching_lines(files: Vec<DirEntry>, regex: &str) -> Vec<Vec<Vec<String>>> {
    let lines =  read_lines_of_files(files,regex);
    return lines;
}
