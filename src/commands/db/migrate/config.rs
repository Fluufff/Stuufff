use serde::Deserialize;

use super::RunInput;
use crate::config::{db::DatabaseConfig, error::RuntimeError, read_config};
#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            database: Default::default(),
        }
    }
}

impl Config {
    pub(super) async fn from_input(input: RunInput) -> Result<Self, RuntimeError> {
        let mut config: Self = read_config(&input.config_file).await?;
        config.database.merge(input.db_args)?;
        Ok(config)
    }
}
