use actix_web::{error, get, web, Error, Result, HttpResponse};
use deadpool_postgres::{Pool, Object};
use tokio_postgres::Error as DbError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

fn db_error<E: std::fmt::Debug>(e: E) -> Error {
  println!("Database error: {:?}", e);
  error::ErrorInternalServerError("Internal server error")
}

#[get("/")]
async fn index(pool: web::Data<Pool>) -> Result<HttpResponse> {
  let conn = pool.get().await.map_err(db_error)?;
  let user = User::find(&conn, "213", "4556").await.map_err(db_error)?;
  println!("{:?}", user);
  Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
struct User {
  id: String,
  name: String,
  email: String,
}

impl User {
  async fn find(conn: &Object, id: &str, email: &str) -> std::result::Result<User, DbError> {
    let stmt = conn.prepare_cached("SELECT $2, $3, $1").await?;
    let row = conn.query_one(&stmt, &[&"some-email", &id, &email]).await?;
    Ok(User {
      id: row.get(0),
      name: row.get(1),
      email: row.get(2),
    })
  }
}
