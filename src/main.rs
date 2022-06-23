extern crate alloc;

use std::fs::File;
use std::io::Write;

use crate::find_files::find_all_files;
use crate::get_source_file_import_lines::find_source_file_import_lines;

mod get_matching_lines;
mod filter_lines;
mod find_files;
mod get_source_file_import_lines;
mod get_dep_file_dep_lines;

fn main() {

  let regex_dep = r#"(compile|testCompile|implementation|testImplementation|compileOnly|runtime|testRuntime
  |[a-zA-Z0-9_]{1,100})\sgroup\:\s['"](.*)['"],\sname\:\s['"](.*)['"],\sversion\:\s['"](.*)['"]
  |(compile|testCompile|implementation|testImplementation|compileOnly|runtime|testRuntime|[a-zA-Z0-9_]{1,100})\s['"](.*):(.*):(.*)['"]
  |(compile|testCompile|implementation|testImplementation|compileOnly|runtime|testRuntime|[a-zA-Z0-9_]{1,100})\(['"](.*):(.*):(.*)['"]\)
  "#;


  let lines = find_source_file_import_lines(
    "C:\\repo\\tptools\\tptool",
    "kt",
    r"^import",
    "import com.opt",
    "import "
  );

  let lines_json = serde_json::to_string_pretty(&lines).unwrap();


  let mut file = File::create("report.json").expect("xxx");

  file.write_all(lines_json.as_ref()).expect("xx");
}





