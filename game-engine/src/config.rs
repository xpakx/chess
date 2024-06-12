use std::env;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    rabbit: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            rabbit: None,
        } 
    } 
}

pub struct ConfigFin {
    pub rabbit: String,
}

fn load_env_config() -> Config {
    Config {
        rabbit: match env::var("RABBIT_URL") {
            Ok(env) => Some(env),
            _ => None,
        },
    }
}

pub fn get_config() -> ConfigFin {
    let env_config = load_env_config();

    ConfigFin {
        rabbit: match env_config.rabbit {
            Some(value) => value,
            None => String::from("amqp://guest:guest@localhost:5672"),
        },
    }
}
