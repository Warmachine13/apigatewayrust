use actix_web::get;

use crate::application;

#[get("/configuration")]
async fn configuration() -> String {
    // req: HttpRequest
    return application::controller::configuration::configuration().await;
}
