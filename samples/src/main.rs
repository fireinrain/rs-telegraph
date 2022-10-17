use std::collections::HashMap;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    println!("同步方式");
    spawn_blocking(move || {request_blocking()});
    // request_blocking();
    Ok(())
}

fn request_blocking() {
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://httpbin.org/post")
        .body("the exact body that is sent")
        .send().unwrap();

    // let result = reqwest::blocking::get("https://httpbin.org/ip").unwrap()
    //     .json::<HashMap<String, String>>().unwrap();

    println!("{:#?}", res);
}


