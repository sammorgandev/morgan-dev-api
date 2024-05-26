use axum::{body::Body, extract::Request, response::Response, Extension, Json};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};

pub async fn create_contact(
    req: Request<Body>,
    Extension(http_client): Extension<Arc<Client>>,
) -> Result<Value, Response> {
    let url = "https://app.loops.so/api/v1/contacts/create";
    let token = match std::env::var("LOOPS_API_KEY") {
        Ok(val) => val,
        Err(_e) => {
            println!("LOOPS_API_KEY not found in .env file");
            return Err(Response::builder()
                .status(400)
                .body(Body::from("LOOPS_API_KEY not found in .env file"))
                .unwrap());
        }
    };

    let mut headers = HeaderMap::new();
    let auth_header_value = match HeaderValue::from_str(&format!("Bearer {}", token)) {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to create auth header: {:?}", e);
            return Err(Response::builder()
                .status(400)
                .body(Body::from("Failed to create auth header"))
                .unwrap());
        }
    };
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, auth_header_value);

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
    let res = client.post(url).headers(headers).json(&data).send().await;
    let _ = match res {
        Ok(response) => {
            let json = response.json::<Value>().await;
            println!("Response: {:?}", json);
            Ok(json.unwrap())
        }
        Err(e) => {
            println!("Failed to send request: {:?}", e);
            Err(Response::builder()
                .status(400)
                .body(Body::from("Failed to send request"))
                .unwrap())
        }
    };

    Ok(json!({
        "message": "Contact created successfully"
    }))
}
