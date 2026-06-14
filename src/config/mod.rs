use std::path::PathBuf;

use serde::de::DeserializeOwned;
use tokio::fs;

pub mod db;
pub mod dynamic_value;
pub mod error;

use error::{MissingConfigError, ParseError, RuntimeError};

pub async fn read_config<C: DeserializeOwned + Default>(
    config_file: &Option<PathBuf>,
) -> Result<C, RuntimeError> {
    let config_file = match config_file {
        Some(c) => c,
        None => return Ok(C::default()),
    };
    let extension = config_file
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(MissingConfigError::NoExtention)?;

    let f = fs::read(config_file)
        .await
        .map_err(|e| MissingConfigError::CannotRead(e))?;
    let config = match extension {
        "yaml" | "yml" => {
            serde_yaml::from_slice(&f).map_err(|e| ParseError::from(e).show_raw(f))?
        }
        "json" => serde_json::from_slice(&f).map_err(|e| ParseError::from(e).show_raw(f))?,
        other => return Err(MissingConfigError::InvalidExtension(other.to_owned()).into()),
    };
    Ok(config)
}
