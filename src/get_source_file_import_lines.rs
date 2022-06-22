use crate::{find_all_files, get_matching_lines};
use crate::find_files::Module;

pub fn find_source_file_import_lines<'a>(
    path_dir: &str,
    file_extension_of_source_files: &str,
    start_of_import_regex: &'a str,
) -> Vec<Module> {
    let source_files = find_all_files(path_dir.clone(), file_extension_of_source_files.clone());

    let matched_source_file_import_lines = get_matching_lines(source_files.clone(), start_of_import_regex.clone());

    return matched_source_file_import_lines;
}