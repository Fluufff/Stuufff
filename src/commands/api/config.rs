use core::ops::Deref;

use serde::Deserialize;
use tracing::instrument;

use super::RunInput;
use crate::config::{
    db::{DatabaseConfig, PsqlPool},
    dynamic_value::DynamicValue,
    error::RuntimeError,
    read_config,
};
#[derive(Deserialize, Clone)]
pub struct OauthConfig {
    pub client: DynamicValue,
    pub secret: DynamicValue,
}

pub const DEFAULT_OAUTH_CLIENT: &str = "???";
pub const DEFAULT_OAUTH_SECRET: &str = "{{ (env('OAUTH_SECRET') or file('/etc/secrets/integrations/oauth') or file('local_secrets/oauth_secret')) | required }}";
impl Default for OauthConfig {
    fn default() -> Self {
        Self {
            client: DynamicValue::new(DEFAULT_OAUTH_CLIENT).unwrap(),
            secret: DynamicValue::new(DEFAULT_OAUTH_SECRET).unwrap(),
        }
    }
}

#[derive(Deserialize, Clone)]
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

    #[instrument(err, skip(self))]
    pub async fn parse(self) -> Result<ParsedConfig, RuntimeError> {
        let psql_pool = self.database.create_pool()?;
        psql_pool.check_schema_updated()?;
        Ok(ParsedConfig {
            orig: self,
            psql_pool,
        })
    }
}

#[derive(Clone)]
pub struct ParsedConfig {
    orig: Config,
    pub psql_pool: PsqlPool,
}
impl Deref for ParsedConfig {
    type Target = Config;
    fn deref(&self) -> &Self::Target {
        &self.orig
    }
}
