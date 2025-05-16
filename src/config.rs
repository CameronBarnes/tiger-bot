use std::env;

use tracing::{error, info};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub database_password: String,
    pub database_name: String,
    pub database_host: String,
    pub database_user: String,
    pub database_port: u16,
    pub cache_host: String,
    pub url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let port = env::var_os("SERVER_PORT").map_or_else(
            || {
                info!("SERVER_PORT environment variable not set. Defaulting to 3000");
                3000
            },
            |port| match port.into_string().map(|str| str.parse::<u16>()) {
                Ok(port) => match port {
                    Ok(port) => port,
                    Err(err) => {
                        error!(
                            "Failed to parse SERVER_PORT environment variable with err: {err:?}"
                        );
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    error!("Failed to parse SERVER_PORT environment variable with err: {err:?}");
                    std::process::exit(1);
                }
            },
        );
        let database_port = env::var_os("DATABASE_PORT").map_or_else(
            || {
                info!("DATABASE_PORT environment variable not set. Defaulting to 5432");
                5432
            },
            |port| match port.into_string().map(|str| str.parse::<u16>()) {
                Ok(port) => match port {
                    Ok(port) => port,
                    Err(err) => {
                        error!(
                            "Failed to parse DATABASE_PORT environment variable with err: {err:?}"
                        );
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    error!("Failed to parse DATABASE_PORT environment variable with err: {err:?}");
                    std::process::exit(1);
                }
            },
        );
        let database_password = env::var("DATABASE_PASSWORD").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                error!("DATABASE_PASSWORD environment variable not set.");
                std::process::exit(1);
            }
            env::VarError::NotUnicode(_) => {
                error!("DATABASE_PASSWORD is not valid unicode. Error: {err:?}");
                std::process::exit(1);
            }
        });
        let url = env::var("URL").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                error!("URL environment variable not set.");
                std::process::exit(1);
            }
            env::VarError::NotUnicode(_) => {
                error!("URL is not valid unicode. Error: {err:?}");
                std::process::exit(1);
            }
        });
        let database_user = env::var("DATABASE_USER").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                info!("DATABASE_USER environment variable not set. Defaulting to postgres");
                String::from("postgres")
            }
            env::VarError::NotUnicode(_) => {
                error!("DATABASE_PASSWORD is not valid unicode. Error: {err:?}");
                std::process::exit(1);
            }
        });
        let database_host = env::var("DATABASE_HOST").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                info!("DATABASE_HOST environment variable not set. Defaulting to 127.0.0.1");
                String::from("127.0.0.1")
            }
            env::VarError::NotUnicode(_) => {
                error!("DATABASE_HOST is not valid unicode. Error: {err:?}");
                std::process::exit(1);
            }
        });
        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                info!("DATABASE_NAME environment variable not set. Defaulting to tigerbot");
                String::from("tigerbot")
            }
            env::VarError::NotUnicode(_) => {
                error!("DATABASE_NAME is not valid unicode. Error: {err:?}");
                std::process::exit(1);
            }
        });
        let cache_host = env::var("CACHE_HOST").unwrap_or_else(|err| match err {
            env::VarError::NotPresent => {
                info!("CACHE_HOST environment variable is not set. Defaulting to cache");
                String::from("cache")
            },
            env::VarError::NotUnicode(_) => {
                error!("CACHE_HOST is not valid unicide. Error: {err:?}");
                std::process::exit(1);
            },
        });
        Self {
            database_password,
            database_port,
            database_host,
            database_user,
            database_name,
            cache_host,
            url,
            port,
        }
    }
}
