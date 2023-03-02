use std::env;
use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let name: String = String::from("OPENAI_API_KEY");
    let OPENAI_API_KEY: String = env::var(name).expect("{name} not found");
    // chaining .await will yield our query result


    let client = reqwest::Client::new();
    let mut data = HashMap::new();

    data.insert("model", "text-davinci-003");
    data.insert("prompt", "Say this is a test");
    data.insert("max_tokens", "7");

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {OPENAI_API_KEY}"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&data)
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}