use crate::{find_all_files, get_matching_lines};

pub fn find_source_file_import_lines(
    path_dir: &str,
    file_extension_of_source_files: &str,
    start_of_import_regex: &str,
) -> Vec<String> {
    let source_files = find_all_files(path_dir, file_extension_of_source_files);

    println!();
    println!("Files with the file extension -{}- : ", &file_extension_of_source_files);
    println!("{:#?}", source_files);

    let matched_source_file_import_lines = get_matching_lines(source_files, start_of_import_regex);

    println!();
    println!("Lines with regex -{}- match  : ", &start_of_import_regex);
    println!("{:#?}", matched_source_file_import_lines);

    return matched_source_file_import_lines;
}