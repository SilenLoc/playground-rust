use std::{env, fs};
use std::error::Error;
use std::path::PathBuf;
use crate::dir_find::find_dir;


pub mod dir_find;

fn main() {
  find_dir(String::from("./"));

}





