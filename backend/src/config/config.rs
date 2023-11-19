use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
#[allow(unused_imports)]
use sea_orm::{Database, DbConn, DbErr};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_host: String,
    pub database_user: String,
    pub database_port: String,
    pub database_password: String,
    pub database_db: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

const POSTGRES_HOSTNAME: &str = "POSTGRES_HOSTNAME";
const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PW: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB: &str = "POSTGRES_DB";
const POSTGRES_PORT: &str = "POSTGRES_PORT";

const JWT_SECRET: &str = "JWT_SECRET";
const JWT_EXPIRY_TIME_MINS: &str = "JWT_EXPIRY_TIME_MINS";
const JWT_MAXAGE: &str = "JWT_MAXAGE";

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

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

        Config {
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

pub async fn get_db_pool(config: &Config) -> Result<DatabaseConnection, DbErr> {
    let opt = ConnectOptions::new(create_db_url(&config));
    let db = Database::connect(opt).await?;

    return Ok(db);
}

pub async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    Migrator::fresh(db).await?;
    Migrator::up(db, None).await?;

    Ok(())
}

fn create_db_url(config: &Config) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_db
    )
}
