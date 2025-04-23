use actix_web::cookie::{time::Duration, Cookie};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

pub fn generate_jwt(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        role: role.to_owned(),
    };
    let secret = env::var("SECRET_KEY").unwrap_or("secret".to_string());
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").unwrap_or("secret".to_string());
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map(|data| data.claims);
    claims
}

pub fn generate_cookie<'a>(
    user_id: &'a str,
    role: &'a str,
) -> Result<Cookie<'a>, jsonwebtoken::errors::Error> {
    generate_jwt(user_id, role).map(|jwt| {
        Cookie::build("Authorization", jwt)
            .path("/")
            .secure(false) // TODO update
            .http_only(true)
            .max_age(Duration::days(24))
            .finish()
    })
}