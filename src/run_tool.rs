use crate::command_runner::run_command;

pub fn run_tool(args: [&str; 1], dir: &str) {
    run_command("tptool", args, dir)
}