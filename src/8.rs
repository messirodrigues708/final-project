use std::process::Command;

fn main() {
    let mut command = Command::new("ls");
    command.arg("-l").arg(".");
    let output = command.output().expect("failed to execute process");
}
