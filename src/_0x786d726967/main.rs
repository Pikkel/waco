/*
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    let target = "https://";
    let response = reqwest::get(target).await?;

    let path = Path::new("./x786d726967");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content =  response.text().await?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
*/
