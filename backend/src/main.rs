#[allow(unused_imports)]
use dotenv::dotenv;
use neodo::config::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();
    let x = &config.database_url;
    
}