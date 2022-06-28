use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

pub fn find_module_refs(
    repo_dir: &str,
    extension_of_dep_files: &str,
    extension_of_src_files: &str,
) -> Vec<ModuleRef> {
    let dep_files = find_all_files(repo_dir, extension_of_dep_files);

    let modules: Vec<&Path> = dep_files.iter().map(|dep_file| dep_file.path().parent().unwrap()).collect();

    let source_files_in_modules: Vec<Vec<DirEntry>> = modules.
        into_iter()
        .map(|module| find_all_files(
            module.as_os_str().to_str().unwrap(), extension_of_src_files)
        ).collect();


    let dep_lines: Vec<Vec<String>> = dep_files.clone().into_iter().map(|dep_file|

        read_lines(dep_file.path()).unwrap().map(|line| line.unwrap_or("no line read".parse().unwrap())).collect()
    ).collect();

    let src_lines: Vec<Vec<Vec<String>>> = source_files_in_modules.into_iter().map(|src_in_module| src_in_module.into_iter().map(|src_file|

        read_lines(src_file.path()).unwrap().map(|line| line.unwrap_or("no line read".parse().unwrap())).collect()
    ).collect()).collect();


    let mut module_refs: Vec<ModuleRef> = Vec::new();

    for (i, dep_file) in dep_files.into_iter().enumerate() {
        module_refs.push(build_module_ref(dep_lines[i].clone(), dep_file.path().parent().unwrap().to_str().unwrap().to_string(), src_lines[i].clone()))
    };

    module_refs
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleRef {
    module_dep_lines: Vec<String>,
    module_path: String,
    lines: Vec<Vec<String>>,
}

impl ModuleRef {
    pub fn filter_src_lines(self, allow_only: &str) -> Self {
        ModuleRef{
            module_dep_lines: self.module_dep_lines,
            module_path: self.module_path,
            lines: self.lines.into_iter().map(|lines_per_file| lines_per_file.into_iter().filter(|line| line.starts_with(&allow_only.to_string())).collect()).collect()
        }
    }
}

fn build_module_ref(module_dep_lines: Vec<String>, module: String, lines: Vec<Vec<String>>) -> ModuleRef {
    ModuleRef {
        module_dep_lines,
        module_path: module.clone(),
        lines,
    }
}


fn find_all_files(path_dir: &str, with_extension: &str) -> Vec<DirEntry> {
    let mut list = Vec::new();

    for file in WalkDir::new(path_dir.clone()).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() && file.path().extension().and_then(OsStr::to_str) == Some(with_extension.clone()) {
            list.push(file.clone())
        }
    };
    return list.to_vec();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}