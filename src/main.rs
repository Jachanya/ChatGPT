use std::env;
use reqwest;
use serde_json::{Value, Map, Number};
use json;


#[tokio::main]
async fn main() {
    let name: String = String::from("OPENAI_API_KEY");
    let openai_api_key: String = env::var(name).expect("{name} not found");
    // chaining .await will yield our query result
    

    let client = reqwest::Client::new();
    let mut data = Map::new();

    data.insert("model".to_string(),    Value::String("text-davinci-003".to_string()));
    data.insert("prompt".to_string(), 
        Value::String("write a love poem".to_string()));
    data.insert("max_tokens".to_string(), Value::Number(Number::from(4000)));

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {openai_api_key}"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&data)
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await
        .unwrap();

    let parsed = json::parse(&response).unwrap();
    
    println!("{}", parsed["choices"][0]);
}