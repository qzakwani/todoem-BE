use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, fmt};
use tracing::{error, info, instrument};

#[derive(Debug, Clone)]
pub struct ConfigError<'a> {
    msg: &'a str,
}

impl fmt::Display for ConfigError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
#[derive(Clone)]
pub struct Config {
    pub pool: PgPool,
    pub port: u16,
    pub secret_key: String,
}

#[instrument]
pub async fn init<'a>() -> Result<Config, ConfigError<'a>> {
    info!("Initializing configuration");
    let db_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => {
            error!("DATABASE_URL not set: {:?}", e);
            return Err(ConfigError {
                msg: "DATABASE_URL not set",
            });
        }
    };

    let secret_key = match env::var("SECRET_KEY") {
        Ok(val) => val,
        Err(e) => {
            error!("SECRET_KEY not set: {:?}", e);
            return Err(ConfigError {
                msg: "SECRET_KEY not set",
            });
        }
    };

    info!("\nConnecting to database ...");
    let pool: PgPool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {:?}", e);
            return Err(ConfigError {
                msg: "Failed to connect to database",
            });
        }
    };

    info!("Running migrations ...");
    if let Err(e) = sqlx::migrate!("src/db/migrations").run(&pool).await {
        error!("Failed to run migrations: {:?}", e);
    } else {
        info!("\n\n\t**Migrations ran successfully**\n\n");
    }

    let config = Config {
        pool,
        port: 8080,
        secret_key,
    };

    Ok(config)
}
