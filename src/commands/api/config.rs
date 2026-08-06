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
    pub enabled: bool,
    pub client: DynamicValue,
    pub secret: DynamicValue,
    pub reader_sa_key: DynamicValue,
}

pub const DEFAULT_OAUTH_CLIENT: &str = "{{ (env('OAUTH_CLIENT') or file('/etc/secrets/integrations/oauth_client') or file('local_secrets/oauth_client')) | required }}";
pub const DEFAULT_OAUTH_SECRET: &str = "{{ (env('OAUTH_SECRET') or file('/etc/secrets/integrations/oauth_secret') or file('local_secrets/oauth_secret')) | required }}";
pub const DEFAULT_READER_KEY: &str = "{{ (env('GOOGLE_READER_KEY_FILE') or filepath('/etc/secrets/integrations/google_reader.json') or filepath('local_secrets/google_reader.json')) | required }}";
impl Default for OauthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client: DynamicValue::new(DEFAULT_OAUTH_CLIENT).unwrap(),
            secret: DynamicValue::new(DEFAULT_OAUTH_SECRET).unwrap(),
            reader_sa_key: DynamicValue::new(DEFAULT_READER_KEY).unwrap(),
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
        config.oauth.enabled = !input.no_auth;
        if config.oauth.enabled {
            if let Some(client) = input.oauth_client_id {
                config.oauth.client = DynamicValue::new_owned(client)?;
            }
            if let Some(secret) = input.oauth_client_secret {
                config.oauth.secret = DynamicValue::new_owned(secret)?;
            }
            if let Some(reader_sa_key) = input.oauth_reader_sa_key {
                config.oauth.reader_sa_key = DynamicValue::new_owned(reader_sa_key)?;
            }
        }
        Ok(config)
    }

    #[instrument(err, skip(self))]
    pub async fn parse(self) -> Result<ParsedConfig, RuntimeError> {
        let psql_pool = self.database.create_pool()?;
        let oauth = match self.oauth.enabled {
            false => None,
            _ => Some(ParsedOauthConfig {
                client: self.oauth.client.get()?,
                secret: self.oauth.secret.get()?,
                google_reader_key_file: self.oauth.reader_sa_key.get()?,
            }),
        };
        let media_folder = self.media_folder.get()?;
        let session_key = generate_pass();
        psql_pool.check_schema_updated()?;

        match fs::metadata(&media_folder).await {
            Ok(s) if s.is_dir() => {}
            Ok(_) => {
                return Err(BadConfigError::from("specified media folder is not a folder").into());
            }
            Err(e) => {
                return Err(BadConfigError::from(format!(
                    "bad media folder '{media_folder}': {e}"
                ))
                .into());
            }
        };

        Ok(ParsedConfig {
            orig: self,
            media_folder,
            psql_pool,
            oauth,
            session_key,
        })
    }
}

#[derive(Clone)]
pub struct ParsedConfig {
    orig: Config,
    pub psql_pool: PsqlPool,
    pub oauth: Option<ParsedOauthConfig>,
    pub session_key: String,
    pub media_folder: String,
}
impl Deref for ParsedConfig {
    type Target = Config;
    fn deref(&self) -> &Self::Target {
        &self.orig
    }
}

#[derive(Clone)]
pub struct ParsedOauthConfig {
    pub client: String,
    pub secret: String,
    pub google_reader_key_file: String,
}

fn generate_pass() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
