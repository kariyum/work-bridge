use crate::services::token::{validate_jwt, Claims};
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::ready;
use crate::error::ServiceError;

impl FromRequest for Claims {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cookie = req
            .cookie("Authorization")
            .map(|token| validate_jwt(token.value()).ok())
            .flatten();
        if let Some(claims) = cookie {
            ready(Ok(claims))
        } else {
            ready(Err(ServiceError::Unauthorized.into()))
        }
    }
}

