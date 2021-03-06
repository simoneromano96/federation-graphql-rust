use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

lazy_static! {
    pub static ref APP_CONFIG: Settings = Settings::init_config();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolConfig {
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostgresConfig {
    pub url: String,
    pub poolsize: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub debug: bool,
    pub accessmodelpath: String,
    pub database: PostgresConfig,
    pub server: ServerConfig,
}

impl Settings {
    fn init_config() -> Self {
        let mut s = Config::default();
        let mut config_file_path = env::current_dir().expect("Cannot get current path");

        // println!("{:?}", config_file_path);

        // Get current RUN_MODE, should be: development/production
        let current_env = env::var("RUN_MODE").unwrap_or(String::from("development"));

        config_file_path.push("environments");
        config_file_path.push(format!("{}.yaml", current_env));

        // println!("{:?}", config_file_path);

        // Add in the current environment file
        // Default to 'development' env
        s.merge(File::from(config_file_path).required(false))
            .expect("Could not read file");

        // Add in settings from the environment
        // DEBUG=1 sets debug key, DATABASE_URL sets database.url key
        s.merge(Environment::new().prefix("APP").separator("_")).expect("Cannot get env");
        
        // println!("{:?}", s);

        // Deserialize configuration
        let r: Settings = s.try_into().expect("Configuration error");


        // Enable all logging
        // if r.debug {
        //     env::set_var("RUST_BACKTRACE", "1");
        //     env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        // }

        // println!("{:?}", r);

        // Should not be necessary
        // if let Ok(database_url) = env::var("DATABASE_URL") {
        //     r.database.url = database_url;
        // }

        r
    }
}
