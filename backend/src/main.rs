use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use error::Result;
#[allow(unused_imports)]
use sea_orm::{Database, DbErr};
use serde_json::{json, Value};

use crate::config::config::{get_app_state, run_migrations};

mod api;
mod config;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    println!("STARTING!!");

    let app_state = get_app_state().await;
    let config = app_state.config;
    let db = app_state.db;

    if !config.is_prod {
        run_migrations(&db).await?;
    }

    let routes = Router::new().route("/health_check", get(health_check));

    Server::bind(&config.socket_address)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

async fn handler_hello() -> impl IntoResponse {
    Html(format!("Hello <strong>!!!</strong>"))
}

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }

// async fn api_login(
// 	payload: Json<LoginPayload>,
// ) -> AppResult<Json<Value>> {
// 	println!("->> {:<12} - api_login", "HANDLER");
// 	// TODO: Implement real db/auth logic.

// 	if payload.username != "demo1" || payload.pwd != "welcome" {
// 		return Err(error::AppError::InternalError);
// 	}

// 	let body = Json(json!({
// 		"result": {
// 			"success": true
// 		}
// 	}));

// 	Ok(body)
// }

// #[derive(Debug)]
// struct LoginPayload {
// 	username: String,
// 	pwd: String,
// }
