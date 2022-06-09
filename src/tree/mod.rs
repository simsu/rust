use actix_web::{web};
mod viewer_campaign;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
  .service(viewer_campaign::campaign)
  .service(test::test_);
}
