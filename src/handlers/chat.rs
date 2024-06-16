use axum::{
    body::{to_bytes, Body},
    extract::Request,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, sync::Arc};

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub stream: bool,
    pub input: Input,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub prompt: String,
    pub prompt_template: String,
}

pub async fn chat_completion(client: Arc<Client>, data: Request<Body>) -> Result<String, Error> {
    dotenv::dotenv().ok();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", env::var("REPLICATE_TOKEN").unwrap())).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let data = to_bytes(data.into_body(), usize::MAX).await;
    let data = data.unwrap_or_else(|err| err.to_string().into());
    let data: Result<serde_json::Value, _> = serde_json::from_slice(data.as_ref());
    let data = data.unwrap_or_else(|err| err.to_string().into());
    let body = json!(data); // Use data instead of body

    let res = client
        .post("https://api.replicate.com/v1/models/meta/meta-llama-3-70b-instruct/predictions")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let prediction: Value = res.json().await?;
    let stream_url = prediction["urls"]["stream"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    Ok(stream_url)
}

// Make a GET request to the stream URL and parse the incoming JSON objects
// let stream = client
//     .get(stream_url)
//     .send()
//     .await
//     .map_err(|_| warp::reject::reject())?
//     .bytes() // Use bytes method here
//     .await
//    .map_err(|_| warp::reject::reject())?;

//let lines: Vec<String> = std::str::from_utf8(&stream)
//   .map_err(|_| warp::reject::reject())?
//  .lines()
//      .map(|line| line.to_string())
//       .collect();

//    let stream = stream::iter(lines.into_iter().map(|line| {
//      let json: serde_json::Value =
//         serde_json::from_str(&line).map_err(|_| warp::reject::reject())?;
//   let data = json["data"].as_str().unwrap();
//     let event = warp::sse::Event::default().data(data);

//   Ok::<_, warp::Rejection>(event)
//  }));
// Ok(stream) // Return Result instead of Ok
