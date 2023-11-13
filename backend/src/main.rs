#[allow(unused_imports)]
use dotenv::dotenv;
use neodo::config::Config;
use sea_orm::{Database, DbErr, EntityTrait};

use entity::user;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    println!("STARTING!!");

    let config = Config::init();
    let x = &config.database_url;

    let db = Database::connect(x).await?;

    // Query For Users
    let users = user::Entity::find().all(&db).await?;

    println!("Users: {:#?}", users);

    Ok(())

}