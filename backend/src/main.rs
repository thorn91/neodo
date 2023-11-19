use axum::{routing::get, Router, Server};
#[allow(unused_imports)]
use sea_orm::{Database, DbErr};

use crate::config::config::{get_db_pool, run_migrations, AppConfig};

mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("STARTING!!");

    let config = AppConfig::init();
    let db = get_db_pool(&config).await?;

    if !config.is_prod {
        run_migrations(&db).await?;
    }

    let routes = Router::new().route("/", get(health_check));

    Server::bind(&config.socket_address)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() {
    println!("HEALTH CHECK");
}

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }
