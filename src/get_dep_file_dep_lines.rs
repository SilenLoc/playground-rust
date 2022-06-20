use crate::{find_all_files, get_matching_lines};

pub fn find_dep_file_dep_lines(
    path_dir: &str,
    file_extension_of_dep_file: &str,
    dep_regex: &str,
) -> Vec<String> {
    let dep_files = find_all_files(path_dir, file_extension_of_dep_file);

    println!();
    println!("Files with the file extension -{}- : ", &file_extension_of_dep_file);
    println!("{:#?}", dep_files);

    let matched_source_file_import_lines = get_matching_lines(dep_files, dep_regex);

    println!();
    println!("Lines with regex -{}- match  : ", &dep_regex);
    println!("{:#?}", matched_source_file_import_lines);

    return matched_source_file_import_lines;
}