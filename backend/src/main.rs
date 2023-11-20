use axum::{
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
mod domain;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    println!("STARTING!!");

    let app_state = get_app_state().await;

    if !app_state.config.is_prod {
        run_migrations(&app_state.db).await?;
    }

    let routes = Router::new().route("/health_check", get(health_check));

    Server::bind(&app_state.config.socket_address)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    let z = domain::user_service::UserService::find(&app_state.db).await?;
    println!("z: {:?}", z);

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
