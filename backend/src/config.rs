#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

const DATABASE_URL: &str = "DATABASE_URL";
const JWT_SECRET: &str = "JWT_SECRET";
const JWT_EXPIRY_TIME_MINS: &str = "JWT_EXPIRY_TIME_MINS";
const JWT_MAXAGE: &str = "JWT_MAXAGE";

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var(DATABASE_URL).expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var(JWT_SECRET).expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var(JWT_EXPIRY_TIME_MINS).expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var(JWT_MAXAGE).expect("JWT_MAXAGE must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}