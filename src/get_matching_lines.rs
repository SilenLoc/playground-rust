use walkdir::DirEntry;
use crate::filter_lines::create_line_regex;
use crate::find_files::{read_lines_of_files};

pub fn get_matching_lines(files: Vec<DirEntry>, regex: &str) -> Vec<Vec<String>> {
    let lines =  read_lines_of_files(files);
    let re = create_line_regex(regex);
    let filtered_lines: Vec<Vec<String>> = lines.into_iter().filter(| tuple| re.is_match(tuple.get(0).unwrap())).collect();

    return filtered_lines;
}
