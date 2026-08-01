use axum::response::{IntoResponse, Response};
use core::{str::FromStr, time::Duration};
use http::StatusCode;
use r2d2::PooledConnection;
use std::fmt::Display;

use diesel::{PgConnection, r2d2::Pool};
use diesel_migrations::MigrationHarness;
use serde::Deserialize;
use tracing::{error, info, instrument};

use crate::config::error::DBConnectError;
use crate::{
    cli::DBArgs,
    config::error::{BadConfigError, RuntimeError},
    psql::{MIGRATIONS, RotatingConnectionManager},
};

use super::dynamic_value::DynamicValue;

/// For use in clap args parsing
/// ```
/// use std::time::Duration;
/// use stuufff::config::shared::parse_duration;
///
/// #[derive(clap::Args)]
/// pub struct MyArgs {
///     #[arg(value_parser=parse_duration)]
///     my_opt: Option<Duration>,
/// }
/// ```
pub fn parse_duration(arg: &str) -> Result<Duration, String> {
    duration_str::parse(arg)
}

#[derive(Debug)]
struct R2D2LogError {}
impl<E: Display> r2d2::HandleError<E> for R2D2LogError {
    fn handle_error(&self, error: E) {
        error!("database error: {error}");
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: DynamicValue,
    pub name: DynamicValue,
    pub user: DynamicValue,
    pub pass: DynamicValue,
    pub port: DynamicValue,
}

pub const DB_DEFAULT_HOST: &str = "{{ env('PSQL_HOST') | default('localhost') }}";
pub const DB_DEFAULT_PORT: &str = "{{ env('PSQL_PORT') | default('5432') }}";
pub const DB_DEFAULT_NAME: &str = "stuufff";
pub const DB_DEFAULT_USER: &str = "{{ env('PSQL_USER') or file('/etc/secrets/psql/user') or file('local_secrets/psql_user') | default('stuufff') }}";
pub const DB_DEFAULT_PASS: &str = "{{ env('PSQL_PASS') or file('/etc/secrets/psql/pass') or file('local_secrets/psql_pass') | required }}";

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: DynamicValue::new(DB_DEFAULT_HOST).unwrap(),
            port: DynamicValue::new(DB_DEFAULT_PORT).unwrap(),
            name: DynamicValue::new(DB_DEFAULT_NAME).unwrap(),
            user: DynamicValue::new(DB_DEFAULT_USER).unwrap(),
            pass: DynamicValue::new(DB_DEFAULT_PASS).unwrap(),
        }
    }
}

impl DatabaseConfig {
    pub fn url(&self) -> Result<String, RuntimeError> {
        let host = self.host.get()?;
        let name = self.name.get()?;
        let user = self.user.get()?;
        let pass = self.pass.get()?;
        let port = self.port()?;

        info!(host, port, name, user, "connecting to db");
        Ok(format!("postgresql://{user}:{pass}@{host}:{port}/{name}"))
    }
}

impl DatabaseConfig {
    pub fn merge(&mut self, args: DBArgs) -> Result<(), BadConfigError> {
        if let Some(db_host) = args.db_host {
            self.host = DynamicValue::new_owned(db_host)?;
        }
        if let Some(db_port) = args.db_port {
            self.port = DynamicValue::new_owned(db_port)?;
        }
        if let Some(db_name) = args.db_name {
            self.name = DynamicValue::new_owned(db_name)?;
        }
        if let Some(db_user) = args.db_user {
            self.user = DynamicValue::new_owned(db_user)?;
        }
        if let Some(db_pass) = args.db_pass {
            self.pass = DynamicValue::new_owned(db_pass)?;
        }
        Ok(())
    }

    fn port(&self) -> Result<u16, RuntimeError> {
        let port = self.port.get()?;
        u16::from_str(&port).map_err(|_| BadConfigError::from("invalid db port").into())
    }

    #[instrument(err, skip(self))]
    pub fn create_pool(&self) -> Result<PsqlPool, RuntimeError> {
        let manager = RotatingConnectionManager::new(self.clone())?;

        let pool = Pool::builder()
            .test_on_check_out(true)
            .error_handler(Box::new(R2D2LogError {}))
            .max_size(2)
            .build(manager)
            .map_err(DBConnectError::from)?;
        let pool = PsqlPool { pool };

        Ok(pool)
    }
}

#[derive(Clone)]
pub struct PsqlPool {
    pub pool: Pool<RotatingConnectionManager<PgConnection>>,
}

impl PsqlPool {
    pub fn connection(
        &self,
    ) -> Result<PooledConnection<RotatingConnectionManager<PgConnection>>, RuntimeError> {
        let conn = self
            .pool
            .get()
            .map_err(DBConnectError::from)
            .map_err(RuntimeError::from)?;
        Ok(conn)
    }

    pub fn connection_or_response(
        &self,
    ) -> Result<PooledConnection<RotatingConnectionManager<PgConnection>>, Response> {
        self.connection()
            .map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "DB unavailable").into_response())
    }

    #[instrument(err, skip(self))]
    pub fn check_schema_updated(&self) -> Result<(), RuntimeError> {
        let conn = &mut self.connection()?;

        let migration_names = conn
            .pending_migrations(MIGRATIONS)
            .map_err(|e| {
                error!("unable to fetch db migration status: {:?}", e.to_string());
                RuntimeError::DBOutdated
            })?
            .into_iter()
            .map(|m| m.name().to_string())
            .collect::<Vec<_>>();
        match migration_names.len() {
            0 => {
                info!("DB seems up to date");
                Ok(())
            }
            _ => {
                error!(
                    "There are pending migrations: {}",
                    migration_names.join(",")
                );
                Err(RuntimeError::DBOutdated)
            }
        }
    }
}
