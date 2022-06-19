use crate::filter_lines::create_line_regex;
use crate::find_files::{find_all_files, read_lines_of_files};


pub mod find_files;
mod filter_lines;

fn main() {
    let file_extension = "rs";
    let path_dir = "./";
    let start_of_import = r"^use";


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
}





