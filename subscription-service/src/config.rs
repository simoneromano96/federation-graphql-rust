use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    env,
    net::IpAddr,
};

lazy_static! {
    pub static ref APP_CONFIG: Settings = Settings::init_config();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfig {
    pub host: IpAddr,
    pub port: u16,
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
    pub redis: RedisConfig,
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
        s.merge(File::from(config_file_path).required(true))
            .expect("Could not read file");

        s.merge(Environment::new().prefix("APP").separator("_")).expect("Cannot merge env");

        // Deserialize configuration
        let mut r: Settings = s.try_into().expect("Configuration error");

        // Enable all logging
        if r.debug {
            env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        }

        // Should not be necessary
        // if let Ok(connection_string) = env::var("MONGO_CONNECTION_STRING") {
        //     r.mongo.connection_string = connection_string;
        // }

        r
    }
}
