use walkdir::DirEntry;
use crate::filter_lines::create_line_regex;
use crate::find_files::{read_lines_of_files};

pub fn get_matching_lines(files: Vec<DirEntry>, regex: &str) -> Vec<String> {
    let lines = read_lines_of_files(files);
    let re = create_line_regex(regex);
    let filtered_lines: Vec<String> = lines.into_iter().filter(|line| re.is_match(line)).collect();

    return filtered_lines;
}