use core::ops::Deref;
use std::collections::HashSet;

use rand::RngExt;
use serde::Deserialize;
use tokio::fs;
use tracing::instrument;

use super::RunInput;
use crate::config::{
    db::{DatabaseConfig, PsqlPool},
    dynamic_value::DynamicValue,
    error::{BadConfigError, RuntimeError},
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
pub struct AccessConfig {
    pub google_roles: GoogleRolesConfig,
}

#[derive(Deserialize, Clone)]
pub struct GoogleRolesConfig {
    pub reader_roles: HashSet<String>,
    pub requester_roles: HashSet<String>,
    pub editor_roles: HashSet<String>,
}

impl Default for GoogleRolesConfig {
    fn default() -> Self {
        Self {
            reader_roles: HashSet::from([
                "04du1wux22l4xt6".into(), // Staff & Volunteers
            ]),
            requester_roles: HashSet::from([
                "00nmf14n0qk8qwh".into(), // Heads and Deputies of Department
            ]),
            editor_roles: HashSet::from([
                "02fk6b3p34i4p1d".into(), // Logistics
                "01baon6m2leb89b".into(), // Web & IT
                                          // "01baon6m3puccov".into(), // Heads
            ]),
        }
    }
}

pub const DEFAULT_MEDIA_FOLDER: &str = "{{ env('MEDIA_FOLDER') or 'media' }}";
#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub oauth: OauthConfig,
    pub access: AccessConfig,
    pub media_folder: DynamicValue,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: Default::default(),
            oauth: Default::default(),
            access: Default::default(),
            media_folder: DynamicValue::new(DEFAULT_MEDIA_FOLDER).unwrap(),
        }
    }
}

impl Config {
    pub(super) async fn from_input(input: RunInput) -> Result<Self, RuntimeError> {
        let mut config: Self = read_config(&input.config_file).await?;
        config.database.merge(input.db_args)?;
        if let Some(media_folder) = input.media_folder {
            config.media_folder = DynamicValue::new_owned(media_folder)?;
        }
        Ok(config)
    }

    #[instrument(err, skip(self))]
    pub async fn parse(self) -> Result<ParsedConfig, RuntimeError> {
        let psql_pool = self.database.create_pool()?;
        let oauth_client = self.oauth.client.get()?;
        let oauth_secret = self.oauth.secret.get()?;
        let media_folder = self.media_folder.get()?;
        let session_key = generate_pass();
        psql_pool.check_schema_updated()?;

        match fs::metadata(&media_folder).await {
            Ok(s) if s.is_dir() => {}
            Ok(_) => {
                return Err(BadConfigError::from("specified media folder is not a folder").into());
            }
            Err(e) => return Err(BadConfigError::from(format!("bad media folder: {e}")).into()),
        };

        Ok(ParsedConfig {
            orig: self,
            media_folder,
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
    pub media_folder: String,
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
