use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: 10000000000, // TODO update expiration time to millisecondspoch
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
    println!("Token is {:?}", token);
    // let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret = env::var("SECRET_KEY").unwrap_or("secret".to_string());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}