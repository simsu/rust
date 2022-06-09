use actix_web::{web};
pub(crate) mod viewer_campaign;
pub(crate) mod viewer;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
  .service(viewer_campaign::campaign)
  .service(test::test_);
}
