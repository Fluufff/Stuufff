use core::ops::Deref;

use rand::RngExt;
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

pub const DEFAULT_OAUTH_CLIENT: &str = "{{ (env('OAUTH_CLIENT') or file('/etc/secrets/integrations/oauth_client') or file('local_secrets/oauth_client')) | required }}";
pub const DEFAULT_OAUTH_SECRET: &str = "{{ (env('OAUTH_SECRET') or file('/etc/secrets/integrations/oauth_secret') or file('local_secrets/oauth_secret')) | required }}";
impl Default for OauthConfig {
    fn default() -> Self {
        Self {
            client: DynamicValue::new(DEFAULT_OAUTH_CLIENT).unwrap(),
            secret: DynamicValue::new(DEFAULT_OAUTH_SECRET).unwrap(),
        }
    }
}

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub database: DatabaseConfig,
    pub oauth: OauthConfig,
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
        let oauth_client = self.oauth.client.get()?;
        let oauth_secret = self.oauth.secret.get()?;
        let session_key = generate_pass();
        psql_pool.check_schema_updated()?;
        Ok(ParsedConfig {
            orig: self,
            psql_pool,
            oauth_client,
            oauth_secret,
            session_key,
        })
    }
}

#[derive(Clone)]
pub struct ParsedConfig {
    orig: Config,
    pub psql_pool: PsqlPool,
    pub oauth_client: String,
    pub oauth_secret: String,
    pub session_key: String,
}
impl Deref for ParsedConfig {
    type Target = Config;
    fn deref(&self) -> &Self::Target {
        &self.orig
    }
}

fn generate_pass() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
