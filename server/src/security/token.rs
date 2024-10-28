use actix_web::cookie::{time::Duration, Cookie};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration, // 24 hours
    };
    // let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
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

pub fn generate_cookie(user_id: &str) -> Result<Cookie, jsonwebtoken::errors::Error> {
    generate_jwt(user_id).map(|jwt| {
        Cookie::build("Authorization", jwt)
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(Duration::hours(24))
            .finish()
    })
}
