use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub fn routes() -> impl HttpServiceFactory {
    web::scope("tasks")
        .route("/{task_id}/proposals/{proposal_id}/messages", web::get().to(crate::routes::messages_handler::get_messages))
}
