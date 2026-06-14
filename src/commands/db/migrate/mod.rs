use std::path::PathBuf;

use diesel_migrations::MigrationHarness;
use tracing::{error, info};

use crate::{cli::DBArgs, config::error::RuntimeError, psql::MIGRATIONS};
mod config;

#[derive(clap::Args)]
pub struct RunInput {
    #[arg(short, long, value_name = "PATH")]
    config_file: Option<PathBuf>,
    #[command(flatten)]
    db_args: DBArgs,
}
pub async fn run(input: RunInput) -> Result<(), RuntimeError> {
    let config = config::Config::from_input(input).await?;

    let pool = config.database.create_pool()?;
    let conn = &mut pool.connection()?;

    conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
        error!("unable to perform db migrations: {:?}", e.to_string());
        RuntimeError::DBOutdated
    })?;

    info!("DB updated");

    Ok(())
}
