extern crate core;

use crate::persistence::{env_default, PersistenceEnv};
use crate::persistence::disk_pers::create_disk_map_env;
use crate::run_tool::run_tool;

mod run_tool;
mod command_runner;
mod persistence;

fn main() {
    let mut disk_map = create_disk_map_env("test.json".to_string(), ".".to_string());

    disk_map.put("hello".to_string(), "world".to_string());

    let result = disk_map.get("hello".to_string());

    println!("{}", result.unwrap())



}





