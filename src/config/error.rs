use std::{fmt::Debug, io};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("no runner specified")]
    NoRunnerSpecified,
    #[error(transparent)]
    BadConfigFile(#[from] MissingConfigError),
    #[error(transparent)]
    BadConfig(#[from] BadConfigError),
    #[error("database has not been fully migrated")]
    DBOutdated,
    #[error(transparent)]
    DBConnectFailure(#[from] DBConnectError),
    #[error(transparent)]
    DBQueryFailure(#[from] diesel::result::Error),
    #[error("{0}: {0:?}")]
    HttpError(#[from] reqwest::Error),
    #[error("google workspace auth error: {0}")]
    GoogleAuthError(#[from] yup_oauth2::Error),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("{0}")]
    JobStopped(&'static str),
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct BadConfigError(String);

impl From<String> for BadConfigError {
    fn from(s: String) -> Self {
        Self(s)
    }
}
impl<'a> From<&'a str> for BadConfigError {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Error)]
pub enum DBConnectError {
    #[error(transparent)]
    PoolConnect(#[from] diesel::result::ConnectionError),
    #[error(transparent)]
    PoolInit(#[from] r2d2::Error),
    #[error(transparent)]
    PoolError(#[from] diesel::r2d2::Error),
    #[error("db connection lost")]
    Disconnected,
}

#[derive(Debug, Error)]
pub enum MissingConfigError {
    #[error("config file path has no extension")]
    NoExtention,
    #[error("config file cannot be read: {0}")]
    CannotRead(#[from] std::io::Error),
    #[error("config file path has an invalid extension {0}")]
    InvalidExtension(String),
}
#[derive(Debug, Error)]
pub enum ParseErrorSource {
    #[error("invalid yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
}
#[derive(Error, Debug)]
pub struct ParseError {
    raw: Option<String>,
    #[source]
    source: ParseErrorSource,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)?;
        if let Some(s) = &self.raw {
            write!(f, ": {}", s)?;
        };
        Ok(())
    }
}

impl From<serde_yaml::Error> for ParseError {
    fn from(source: serde_yaml::Error) -> Self {
        Self {
            raw: None,
            source: source.into(),
        }
    }
}
impl From<serde_json::Error> for ParseError {
    fn from(source: serde_json::Error) -> Self {
        Self {
            raw: None,
            source: source.into(),
        }
    }
}

impl ParseError {
    pub fn show_raw(mut self, raw: Vec<u8>) -> Self {
        let s = String::from_utf8(raw).unwrap_or("invalid UTF-8".into());
        self.raw = Some(s);
        self
    }
}
