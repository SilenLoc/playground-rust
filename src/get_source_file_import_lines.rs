use crate::{find_all_files, get_matching_lines};

pub fn find_source_file_import_lines<'a>(
    path_dir: &str,
    file_extension_of_source_files: &str,
    start_of_import_regex: &'a str,
) -> Vec<Vec<String>> {
    let source_files = find_all_files(path_dir, file_extension_of_source_files);

    let matched_source_file_import_lines = get_matching_lines(source_files, start_of_import_regex);

    return matched_source_file_import_lines;
}