use axum::http::StatusCode;
use axum::Json;
use hyper::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

//function to take in a username & password and return an jsonwebtoken.
pub async fn login_handler(
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = &login_info.username;
    let password = &login_info.password;
    let is_valid = is_valid_user(username, password);

    if is_valid {
        let claims = Claims {
            sub: username.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };
        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        ) {
            Ok(t) => t,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        return Ok(Json(LoginResponse { token }));
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

//function to validate a username and password. Right now it just checks for admin / password as static strings.
pub fn is_valid_user(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

//function to submit the jsonwebtoken returned from login_handler and validate the authorization bearer token. Returns OK or Err.
pub async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => {
                        let info = "You are valid here is Info".to_string();
                        Ok(Json(info))
                    }
                    Err(e) => {
                        eprintln!("Failed to decode token: {:?}", e);
                        Err(StatusCode::UNAUTHORIZED)
                    }
                }
            } else {
                eprintln!("Failed to decode token: {:?}", auth_header_str);
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            eprintln!("Failed to decode token: {:?}", auth_header.to_str());
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        eprintln!(
            "Failed to decode token: {:?}",
            header_map.get("Authorization")
        );
        Err(StatusCode::UNAUTHORIZED)
    }
}
