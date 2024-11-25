use actix_web::{
    cookie::{time::Duration, Cookie},
    get, HttpResponse, Responder,
};

#[get("/logout")]
pub async fn logout() -> impl Responder {
    let mut response = HttpResponse::NoContent().finish();
    let cookie = Cookie::build("Authorization", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();
    response.add_cookie(&cookie).expect("Failed to remove cookie");
    response
}