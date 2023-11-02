use redis::{Connection, RedisResult, Commands, Client};
use serde_derive::Deserialize;

// Define a struct to hold our configuration values 
#[derive(Deserialize)]
pub struct Config {
    pub redis: RedisConfig,
}
// Define a struct to hold our Redis configuration values
#[derive(Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: Option<String>,
}

// Read config file
fn read_config() -> Config {
    let config_value: String = std::fs::read_to_string("config.toml")
        .expect("Unable to read config file");
    let config: Config = toml::from_str(&config_value).expect("Unable to parse config file");
    config
}

// Define a struct to hold Redis connection
pub struct RedisInstance {
    pub config: Config,
    pub connection: Connection,
    init: bool,
}

// Implement RedisInstance
impl RedisInstance {
    // new a RedisInstance struct and connect to Redis
    pub fn new() -> RedisInstance {
        let config = read_config();
        let connection_str = format!("redis://{}:{}/{}", config.redis.host, config.redis.port, config.redis.db);
        let connection = Client::open(connection_str.as_str())
            .expect("Unable to open Redis connection")
            .get_connection()
            .expect("Unable to get Redis connection");
        RedisInstance {
            config,
            connection,
            init: true,
        }
    }

    // check if redis is initialized return a RedisResult
    fn check_init(&self) -> RedisResult<bool> {
        if self.init {
            RedisResult::Ok(true)
        } else {
            Err(redis::RedisError::from((
                redis::ErrorKind::InvalidClientConfig,
                "Redis not initialized",
            )))
        }
    }

    // get a value from Redis
    pub fn get(&mut self, key: &str) -> RedisResult<String> {
        // check if Redis is initialized
        self.check_init()?;
        self.connection.get(key)
    }

    // set a value to Redis
    pub fn set(&mut self, key: &str, value: &str) -> RedisResult<()> {
        self.check_init()?;
        self.connection.set(key, value)
    }
}