use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use url::Url;
use std::{env, net::IpAddr, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

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
pub struct SessionConfig {
    pub secret: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoConfig {
    pub connection_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationServerConfig {
    pub basic_auth: BasicAuthConfig,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub debug: bool,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub mongo: MongoConfig,
    pub server: ServerConfig,
    pub authorization_server: AuthorizationServerConfig,
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
        s.merge(File::from(config_file_path).required(true)).expect("Could not read file");

        // Deserialize configuration
        let r: Settings = s.try_into().expect("Configuration error");
        
        // Enable all logging
        if r.debug {
            env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        }

        r
    }
}
