#[allow(unused_imports)]
use sea_orm::{Database, DbErr};

mod config;
mod api;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("STARTING!!");


    Ok(())
}