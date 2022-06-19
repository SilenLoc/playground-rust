use std::fs;
use std::path::PathBuf;

pub fn find_dir(asked_dir_path: String) -> PathBuf {
    let out_dir: String = asked_dir_path.clone();

    for entry in fs::read_dir(&out_dir).unwrap() {
        let path = entry.unwrap().path();
        println!("content of {} {} {}", &out_dir, ": ", path.file_name().unwrap().to_str().unwrap());
    }

    let path_entry = fs::read_dir(&out_dir);
    let path_buf = path_entry.unwrap().last().unwrap().unwrap().path();

    return path_buf;
}

