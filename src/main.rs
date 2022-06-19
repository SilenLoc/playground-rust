use crate::get_matching_lines::get_matching_lines;

mod get_matching_lines;
mod filter_lines;
mod find_files;

fn main() {
    let file_extension_of_source_files = "rs";

    let file_extension_of_dep_file = ".toml";

    let path_dir = "./";
    let start_of_import = r"^use";

    let matched_lines = get_matching_lines(file_extension_of_source_files, path_dir, start_of_import);
}





