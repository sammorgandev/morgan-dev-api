use axum::{
    body::{to_bytes, Body},
    extract::Request,
    response::Response,
    Extension,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;

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

    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let data: Value = serde_json::from_slice(&body).unwrap();

    let client = http_client;
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

pub async fn contact_form(
    req: Request<Body>,
    Extension(http_client): Extension<Arc<Client>>,
) -> Result<Value, Response> {
    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let data: Value = serde_json::from_slice(&body).unwrap();

    let transactional_id = "clwoat3c0000qm6x1htkkwgcb";
    let first = data["first"].as_str().unwrap();
    let last = data["last"].as_str().unwrap();
    let email = data["email"].as_str().unwrap();
    let message = data["message"].as_str().unwrap();
    let data_variables = json!({
        "first": first,
        "last": last,
        "email": email,
        "message": message,
    });

    let api_body = json!({
        "transactionalId": transactional_id,
        "email": "sam@morgan.dev",
        "dataVariables": data_variables,
    });

    let url = "https://app.loops.so/api/v1/transactional";
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

    let client = http_client;
    let res = client
        .post(url)
        .headers(headers)
        .json(&api_body)
        .send()
        .await;
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
