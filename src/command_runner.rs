use std::env;
use std::path::Path;
use std::process::Command;

pub fn run_command(command: &str, args: [&str;1], dir: &str) {
    let target = Path::new(dir);
    env::set_current_dir(target).expect("changing to dir did not work");


    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .args(args)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .args(args)
            .output()
            .expect("failed to execute process")
    };

    let out = String::from_utf8(output.stdout).unwrap();

    println!("{}", out)
}