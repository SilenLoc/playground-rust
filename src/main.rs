use std::fs::File;
use std::io::Write;

use crate::find_files::find_all_files;
use crate::get_matching_lines::get_matching_lines;
use crate::get_source_file_import_lines::find_source_file_import_lines;

mod get_matching_lines;
mod filter_lines;
mod find_files;
mod get_source_file_import_lines;
mod get_dep_file_dep_lines;

fn main() {
  let lines = find_source_file_import_lines("./somerepo", "txt", r"^someimport");

  let lines_json = serde_json::to_string_pretty(&lines).unwrap();

  println!("{}", lines_json);

  let mut file = File::create("report.json").expect("xxx");

  file.write_all(lines_json.as_ref()).expect("xx");
}





