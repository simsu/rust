use chrono::prelude::*;
use actix_web::{error, get, web, Error, Result, HttpResponse};
use tokio_postgres::Error as DbError;
use deadpool_postgres::{Pool, Object};
use serde::Serialize;
use chrono::serde::ts_milliseconds;

//에러가 나융
mod viewer;
use viewer::Viewer;

fn db_error<E: std::fmt::Debug>(e: E) -> Error {
  println!("Database error: {:?}", e);
  error::ErrorInternalServerError("Internal server error")
}

// pub struct Viewer {}

// impl Viewer {
//   pub fn return_id() -> String {
//     let id = "id-1";
//     id.to_owned()
//   }
// }

#[get("/campaign")]
async fn campaign(pool: web::Data<Pool>) -> Result<HttpResponse> {
  let conn = pool.get().await.map_err(db_error)?;
  let data = ViewerCampaign::find(&conn, &Viewer::return_id()).await.map_err(db_error)?;
  println!("{:?}", data);
  Ok(HttpResponse::Ok().json(data))
}

#[derive(Debug, Serialize)]
struct ViewerCampaign {
  id: String,
}

impl ViewerCampaign {
  async fn find(conn: &Object, id: &str) -> std::result::Result<Vec<Self>, DbError> {
    let stmt = conn.prepare_cached("SELECT id FROM viewers").await?;
    let rows = conn.query(&stmt, &[&id]).await?;
    let campaigns = rows
      .into_iter()
      .map(|row| Self {
        id: row.get(0),
      })
      .collect();
    Ok(campaigns)
  }
}
