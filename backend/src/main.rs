#[allow(unused_imports)]
use sea_orm::{Database, DbErr};

use crate::config::config::{Config, get_db_pool, run_migrations};

mod config;
mod api;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("STARTING!!");

    let config = Config::init();
    let db = get_db_pool(&config).await?;
    run_migrations(&db).await?;
    
    Ok(())
}