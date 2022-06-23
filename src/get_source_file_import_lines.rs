use crate::find_all_files;
use crate::find_files::{Module, read_lines_of_files};

pub fn find_source_file_import_lines<'a>(
    path_dir: &str,
    file_extension_of_source_files: &str,
    start_of_import_regex: &'a str,
    ignore_import_that_start_with: &'a str,
    cut_from_dep: &'a str
) -> Vec<Module> {
    let source_files = find_all_files(path_dir.clone(), file_extension_of_source_files.clone());

    let matched_source_file_import_lines = read_lines_of_files(
        source_files.clone(),
        start_of_import_regex.clone(),
        ignore_import_that_start_with,
        cut_from_dep
    );

    return matched_source_file_import_lines;
}