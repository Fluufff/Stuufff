use diesel::r2d2::{ConnectionManager, R2D2Connection};
use diesel_migrations::embed_migrations;
use r2d2::ManageConnection;

use crate::config::{
    db::DatabaseConfig,
    error::{DBConnectError, RuntimeError},
};

pub mod models;
pub mod schema;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    embed_migrations!("src/psql/migrations");

pub type Connection = r2d2::PooledConnection<RotatingConnectionManager<diesel::PgConnection>>;

#[derive(Clone, Debug)]
pub struct RotatingConnectionManager<T> {
    config: DatabaseConfig,
    manager: ConnectionManager<T>,
}

impl<T> RotatingConnectionManager<T> {
    pub fn new(config: DatabaseConfig) -> Result<Self, RuntimeError> {
        let url = config.url()?;
        let manager = ConnectionManager::new(url);
        Ok(Self { config, manager })
    }
}

impl<T> ManageConnection for RotatingConnectionManager<T>
where
    T: R2D2Connection + Send + 'static,
{
    type Connection = T;
    type Error = RuntimeError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let url = self.config.url()?;
        T::establish(&url)
            .map_err(DBConnectError::from)
            .map_err(RuntimeError::from)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        self.manager
            .is_valid(conn)
            .map_err(DBConnectError::from)
            .map_err(RuntimeError::from)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.manager.has_broken(conn)
    }
}
