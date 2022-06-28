use std::fs::File;
use std::io::Write;

use crate::modules_finder::{find_module_refs, ModuleRef};

mod modules_finder;

fn main() {
  let pure_lines = find_module_refs("C:\\repo\\tptools\\tptool", "kts", "kt");

  let lines: Vec<ModuleRef> = pure_lines.into_iter().map(|module_ref| module_ref.filter_src_lines("import ")).collect();

  let lines_json = serde_json::to_string_pretty(&lines).unwrap();


  let mut file = File::create("report.json").expect("xxx");

  file.write_all(lines_json.as_ref()).expect("xx");
}





