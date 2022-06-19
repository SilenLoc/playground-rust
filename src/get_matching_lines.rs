use crate::filter_lines::create_line_regex;
use crate::find_files::{find_all_files, read_lines_of_files};

pub fn get_matching_lines(file_extension: &str, path_dir: &str, start_of_import: &str) -> Vec<String> {

    let files = find_all_files(path_dir, file_extension);

    println!();
    println!("Files with the file extension -{}- : ", &file_extension);
    println!("{:#?}", files);

    let lines = read_lines_of_files(files);
    let re = create_line_regex(start_of_import);
    let filtered_lines: Vec<String> = lines.into_iter().filter(|line| re.is_match(line)).collect();

    println!();
    println!("Lines with regex -{}- match  : ", &start_of_import);
    println!("{:#?}", filtered_lines);

    return filtered_lines
}