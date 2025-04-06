use crate::application::check_health::check_health;
use actix_web::{HttpResponse, Responder, get, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[get("/health")]
async fn health_check() -> impl Responder {
    match check_health().await {
        Ok(ok) => {
            return HttpResponse::Ok().json(ok);
        }
        Err(err) => {
            return HttpResponse::ServiceUnavailable().json(err);
        }
    }
}
