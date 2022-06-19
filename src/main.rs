use crate::find_files::find_all_files;
use crate::get_dep_file_dep_lines::find_dep_file_dep_lines;
use crate::get_matching_lines::get_matching_lines;
use crate::get_source_file_import_lines::find_source_file_import_lines;

mod get_matching_lines;
mod filter_lines;
mod find_files;
mod get_source_file_import_lines;
mod get_dep_file_dep_lines;

fn main() {
    find_source_file_import_lines("./", "rs", r"^use");
    find_dep_file_dep_lines("./", "toml", r"^use");
}





