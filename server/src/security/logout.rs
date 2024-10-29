use actix_web::{
    cookie::{time::Duration, Cookie},
    get, HttpRequest, HttpResponse, Responder,
};

#[get("/logout")]
pub async fn logout2(request: HttpRequest) -> impl Responder {
    let mut response = HttpResponse::Ok().finish();
    let cookie_option = request.cookie("Authorization");
    if let Some(cookie) = cookie_option {
        println!("Cookie is {:?}", cookie);
        response
            .add_removal_cookie(&cookie)
            .expect("Failed to remove cookie");
    }
    println!("Cookie is {:?}", response);
    response
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    let mut response = HttpResponse::Ok().finish();
    let cookie = Cookie::build("Authorization", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();
    response.add_cookie(&cookie).expect("Failed to remove cookie");
    println!("Response is {:?}", response);
    response
}
