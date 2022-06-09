use std::env;
use actix_web::{web, App, HttpServer};
use deadpool_postgres::{Config, ManagerConfig, Pool, Runtime};
use tokio_postgres::NoTls;

mod http;
mod tree;

fn env(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(text) => Some(text),
        _ => None,
    }
}

fn establish_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = env("DB_NAME");
    cfg.user = env("DB_USERNAME");
    cfg.password = env("DB_PASSWORD");
    cfg.host = env("DB_HOST");
    cfg.manager = Some(ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("DB Connection Failed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("DB_NAME", "");
    env::set_var("DB_USERNAME", "");
    env::set_var("DB_PASSWORD", "");
    env::set_var("DB_HOST", "");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(establish_pool()))
            .configure(http::config)
            .service(web::scope("/test").configure(tree::config))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}