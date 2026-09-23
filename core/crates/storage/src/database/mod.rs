pub mod api_clients;
pub mod assets;
pub mod assets_addresses;
pub mod assets_associations;
pub mod assets_links;
pub mod assets_usage_ranks;

pub mod chains;
pub mod charts;
pub mod config;
pub mod devices;
pub mod fiat;
pub mod migrations;
pub mod nft;
pub mod notifications;
pub mod parser_state;
pub mod perpetuals;
pub mod price_alerts;
pub mod prices;
pub mod prices_providers;
pub mod referrals;
pub mod releases;
pub mod rewards;
pub mod rewards_redemptions;
pub mod scan_addresses;
pub mod scan_detections;
pub mod support_sessions;
pub mod tag;
pub mod transactions;
pub mod usernames;
pub mod wallets;

use diesel::connection::{AnsiTransactionManager, TransactionManager};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

use crate::DatabaseError;

pub fn create_pool(database_url: &str, pool_size: u32) -> Result<PgPool, DatabaseError> {
    if pool_size == 0 {
        return Err(DatabaseError::ConnectionPool);
    }
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(pool_size).build(manager).map_err(|_| DatabaseError::ConnectionPool)
}

pub struct DatabaseClient {
    connection: PgPooledConnection,
}

impl DatabaseClient {
    pub fn from_pool(pool: &PgPool) -> Result<Self, DatabaseError> {
        let connection = pool.get().map_err(|_| DatabaseError::ConnectionPool)?;
        Ok(Self { connection })
    }

    pub(crate) fn transaction<T, E: From<DatabaseError>>(&mut self, operation: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E> {
        AnsiTransactionManager::begin_transaction(&mut *self.connection).map_err(DatabaseError::from)?;
        match operation(self) {
            Ok(value) => {
                AnsiTransactionManager::commit_transaction(&mut *self.connection).map_err(DatabaseError::from)?;
                Ok(value)
            }
            Err(error) => {
                AnsiTransactionManager::rollback_transaction(&mut *self.connection).map_err(DatabaseError::from)?;
                Err(error)
            }
        }
    }
}
