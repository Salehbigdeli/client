use std::fs;
use std::time::SystemTime;
use reqwest::Client;
use tokio;
use futures::{FutureExt};
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use bytes;
use serde_json;

async fn req(client: &Client, url: &str, payload: &serde_json::Value) -> Result<bytes::Bytes, reqwest::Error>{
    let resp = client.get(url)
                .json(payload).send().await?;
    resp.bytes().await
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let start = SystemTime::now();


    // let payload = r#""#;
    let payload_path = "payload.json";
    let payload = fs::read_to_string(payload_path).unwrap_or_else(|e| {
        println!(r#"please fill {} otherwise defaults to hello world"#, payload_path);
        eprintln!("error {}, {}", e, payload_path);
        String::from(r#"{
        "hello": "world"
        }"#)
    });

    let payload: serde_json::Value =
        serde_json::from_str(&payload).expect("JSON was not well-formatted");

    let url_path = "address";
    let count_url = fs::read_to_string(url_path).unwrap_or_else(|e| {
        println!(r#"please fill "{}" otherwise defaults to "2
        http://localhost:8080""#, url_path);
        eprintln!("error {}, {}", e, url_path);
        String::from(r#"2
        http://localhost:8080"#)
    });
    let mut count_url = count_url.split("\n");
    let req_count: i32 = count_url.next().unwrap().trim().parse().unwrap();
    let  url = count_url.next().unwrap().trim();

    let bodies: FuturesUnordered<_> = (0..req_count).map(|i| {
        let client = &client;
        let payload = &payload;
        println!("{}", i);
        req(client, url, payload).boxed()
    }).into_iter().collect();
    let bodies: Vec<_> = bodies.collect().await;
    let mut fail_count = 0;
    let mut succ_cout = 0;
    for b in bodies {
        match b {
            Ok(_b) => {
                // println!("Got {} bytes", b.len());
                succ_cout += 1;
            },
            Err(e) => {
                eprintln!("Got an error: {}", e);
                fail_count += 1;
            },
        }
    }

    let elapsed_secs = start.elapsed().unwrap().as_millis() as f64/1000.0;
    println!("spent {}s",  elapsed_secs);
    println!("{} reqs/sec", req_count as f64 /elapsed_secs);
    println!("failed {}\nsucceed {}", fail_count, succ_cout);
}
