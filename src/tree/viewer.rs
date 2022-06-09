use chrono::prelude::*;
use actix_web::{error, get, web, Error, Result, HttpResponse};
use tokio_postgres::Error as DbError;
use deadpool_postgres::{Pool, Object};
use serde::Serialize;
use chrono::serde::ts_milliseconds;

pub struct Viewer {}

impl Viewer {
  pub fn return_id() -> String {
    let id = "id-1";
    id.to_owned()
  }
}