use std::process::Command;

pub fn main() {
    let gen = Command::new("echo")
        .arg("Hello world")
        .spawn()
        .expect("Failed to execute command");
}
