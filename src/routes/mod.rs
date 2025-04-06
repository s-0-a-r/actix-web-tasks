mod health;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    health::config(cfg);
}
