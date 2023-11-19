use std::net::SocketAddr;

use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
#[allow(unused_imports)]
use sea_orm::{Database, DbConn, DbErr};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub is_prod: bool,

    pub socket_address: SocketAddr,

    pub database_host: String,
    pub database_user: String,
    pub database_port: String,
    pub database_password: String,
    pub database_db: String,

    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

const IS_PROD: &str = "IS_PROD";

const HOST_ADDR: &str = "HOST_ADDR";
const HOST_PORT: &str = "HOST_PORT";

const POSTGRES_HOSTNAME: &str = "POSTGRES_HOSTNAME";
const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PW: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB: &str = "POSTGRES_DB";
const POSTGRES_PORT: &str = "POSTGRES_PORT";

const JWT_SECRET: &str = "JWT_SECRET";
const JWT_EXPIRY_TIME_MINS: &str = "JWT_EXPIRY_TIME_MINS";
const JWT_MAXAGE: &str = "JWT_MAXAGE";

impl AppConfig {
    pub fn init() -> AppConfig {
        dotenv().ok();

        let is_prod = std::env::var(IS_PROD)
            .map(|x| x == "true")
            .expect("IS_PROD must be set");

        let host_addr = std::env::var(HOST_ADDR).expect("HOST_ADDR must be set");
        let host_port = std::env::var(HOST_PORT).expect("HOST_PORT must be set");

        let database_host =
            std::env::var(POSTGRES_HOSTNAME).expect("POSTGRES_HOSTNAME must be set");
        let database_user = std::env::var(POSTGRES_USER).expect("POSTGRES_USER must be set");
        let database_password = std::env::var(POSTGRES_PW).expect("POSTGRES_PW must be set");
        let database_db = std::env::var(POSTGRES_DB).expect("POSTGRES_DB must be set");
        let database_port = std::env::var(POSTGRES_PORT).expect("POSTGRES_PORT must be set");

        let jwt_secret = std::env::var(JWT_SECRET).expect("JWT_SECRET must be set");
        let jwt_expires_in =
            std::env::var(JWT_EXPIRY_TIME_MINS).expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var(JWT_MAXAGE).expect("JWT_MAXAGE must be set");

        let socket_address = get_socket_address(host_addr, host_port);

        AppConfig {
            is_prod,

            socket_address,

            database_host,
            database_user,
            database_port,
            database_password,
            database_db,

            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}

fn get_socket_address(address: String, port: String) -> SocketAddr {
    let addr = format!("{}:{}", address, port);
    addr.parse::<SocketAddr>().expect("Could not create socket address")
}

pub async fn get_db_pool(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let opt = ConnectOptions::new(create_db_url(&config));
    let db = Database::connect(opt).await?;

    return Ok(db);
}

pub async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    Migrator::fresh(db).await?;
    Migrator::up(db, None).await?;

    Ok(())
}

fn create_db_url(config: &AppConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_db
    )
}
