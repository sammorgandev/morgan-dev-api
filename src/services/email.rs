use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;

pub async fn create_contact() -> Result<(), Box<dyn Error>> {
    let url = "https://app.loops.so/api/v1/contacts/create";
    let token = std::env::var("LOOPS_API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let data = json!({
        "email": "<string>",
        "firstName": "<string>",
        "lastName": "<string>",
        "source": "<string>",
        "subscribed": true,
        "userGroup": "<string>",
        "userId": "<string>"
    });

    let client = reqwest::Client::new();
    let res = client.post(url).headers(headers).json(&data).send().await?;

    println!("Response: {:?}", res);
    Ok(())
}
