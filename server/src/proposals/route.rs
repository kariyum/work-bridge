use actix_web::dev::HttpServiceFactory;
use actix_web::web;

use crate::proposals::repo::delete_proposal;
use crate::proposals::repo::get_proposal;
use crate::proposals::repo::get_proposals;
use crate::proposals::repo::create_proposal;

pub fn proposal_routes() -> impl HttpServiceFactory {
    web::scope("")
        .service(create_proposal)
        .service(get_proposals)
        .service(delete_proposal)
        .service(get_proposal)
}

// pub fn proposal_routessss(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/proposals")
//             .route("", web::get().to(get_proposals))
//             .service(delete_proposal)
//             .service(get_proposal),
//     );
// }
