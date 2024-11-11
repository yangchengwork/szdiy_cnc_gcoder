use std::env;
// use thiserror::Error;
use std::collections::HashMap;
use reqwest::{Client, Response, Error};

pub async fn sentiment(url: &str, sentence: &str) -> Result<Response, Error> {
    let token = env::var("HF_TOKEN").expect("need HF_TOKEN");

    let mut map = HashMap::new();
    map.insert("inputs", sentence);
    // info!("question is {sentence}");
    let client = Client::new();
    let res = client.post(url)
        .header("Authorization", "Bearer ".to_string() + token.as_str())
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await?;

    Ok(res)
}