use std::{env, net::SocketAddr};

use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
#[allow(unused_imports)]
use sea_orm::{Database, DbConn, DbErr};
use std::sync::Arc;

use crate::error::Error;

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,
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

        let is_prod = env::var(IS_PROD)
            .map(|x| x == "true")
            .map_err(|_| Error::InvalidSetup("IS_PROD must be set".to_owned()))
            .unwrap();

        let host_addr = env::var(HOST_ADDR)
            .map_err(|_| Error::InvalidSetup("HOST_ADDR must be set".to_owned()))
            .unwrap();
        let host_port = env::var(HOST_PORT)
            .map_err(|_| Error::InvalidSetup("HOST_PORT must be set".to_owned()))
            .unwrap();

        let database_host = env::var(POSTGRES_HOSTNAME)
            .map_err(|_| Error::InvalidSetup("POSTGRES_HOSTNAME must be set".to_owned()))
            .unwrap();
        let database_user = env::var(POSTGRES_USER)
            .map_err(|_| Error::InvalidSetup("POSTGRES_USER must be set".to_owned()))
            .unwrap();
        let database_password = env::var(POSTGRES_PW)
            .map_err(|_| Error::InvalidSetup("POSTGRES_PW must be set".to_owned()))
            .unwrap();
        let database_db = env::var(POSTGRES_DB)
            .map_err(|_| Error::InvalidSetup("POSTGRES_DB must be set".to_owned()))
            .unwrap();
        let database_port = env::var(POSTGRES_PORT)
            .map_err(|_| Error::InvalidSetup("POSTGRES_PORT must be set".to_owned()))
            .unwrap();

        let jwt_secret = env::var(JWT_SECRET)
            .map_err(|_| Error::InvalidSetup("JWT_SECRET must be set".to_owned()))
            .unwrap();
        let jwt_expires_in = env::var(JWT_EXPIRY_TIME_MINS)
            .map_err(|_| Error::InvalidSetup("JWT_EXPIRY_TIME_MINS must be set".to_owned()))
            .unwrap();
        let jwt_maxage = env::var(JWT_MAXAGE)
            .map_err(|_| Error::InvalidSetup("JWT_MAXAGE must be set".to_owned()))
            .unwrap();

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

pub async fn get_app_state() -> AppState {
    let config = AppConfig::init();
    let db: DatabaseConnection = get_db_pool(&config).await.unwrap();
    AppState { config, db }
}

fn get_socket_address(address: String, port: String) -> SocketAddr {
    let addr = format!("{}:{}", address, port);
    addr.parse::<SocketAddr>()
        .map_err(|e| Error::InvalidSetup((e.to_string().to_owned())))
        .unwrap()
}

async fn get_db_pool(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
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
