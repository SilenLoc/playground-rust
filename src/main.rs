use crate::filter_lines::create_line_regex;
use crate::find_files::{find_all_files, read_lines_of_files};


pub mod find_files;
mod filter_lines;

fn main() {
    let files = find_all_files("./", "rs");

    let lines = read_lines_of_files(files);

    let re = create_line_regex(r"^use");

    let filtered_lines: Vec<String> = lines.into_iter().filter(|line| re.is_match(line)).collect();

    println!("{:#?}", filtered_lines)
}





