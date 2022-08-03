use crate::persistence::{env_default, PersistenceEnv};
use crate::run_tool::run_tool;

mod run_tool;
mod command_runner;
mod persistence;

fn main() {
    let save_env: Box<PersistenceEnv> = env_default();
    save_env.save_to_local("hey");
}





