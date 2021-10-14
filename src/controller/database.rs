use std::time::Duration;
use diesel::{SqliteConnection, connection::SimpleConnection, r2d2::{ConnectionManager, Pool}};
use dotenv::dotenv;
use log::error;
use std::env;

pub mod ingredient;

trait CRUDController {

    type NewT;
    type T;

    /**
     * Connects to database and returns active connection.
     *
     * Currently only sqlite is supported.
     *
     * # Attributes
     * * `database_url` - String describing path to database
     *
     * # Returns
     * * Pool containing the connection to given sqlite database on success
     * * None on Error
     */
    fn connect_database(database_url: &str)
        -> Option<Pool<ConnectionManager<SqliteConnection>>> 
    {
        match Pool::builder()
            .max_size(16)
            .connection_customizer(Box::new(ConnectionOptions {
                enable_wal: true,
                enable_foreign_keys: true,
                busy_timeout: Some(Duration::from_secs(30))
            }))
            .build(ConnectionManager::<SqliteConnection>::new(database_url)) {
                Ok(db_pool) => Some(db_pool),
                Err(e) => {
                    error!("Could not connect to database: {}", e);
                    None
                }
            }
    }

    fn create(&self, new_item: &Self::NewT) -> bool;
    fn read(&self, item_id: i32) -> Option<Self::T>;
    fn update(&self, item_id: i32, item: Self::T) -> bool;
    fn delete(&self, item_id: i32) -> bool;
} 

/**
 * Object containing settings for database access.
 */
#[derive(Debug)]
struct ConnectionOptions
{
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions
{
    /**
     * Sets settings for database access when a sqlite connection is acquired.
     */
    fn on_acquire(&self, conn: &mut SqliteConnection)
        -> Result<(), diesel::r2d2::Error>
    {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {}", d.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

/**
 * Read local environment variable describing path to database
 *
 * Currently only sqlite is supported.
 *
 * # Returns
 *
 * * String on success
 * * None on error
 */
pub fn local_conn_string() -> Option<String> {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(db_str) => Some(db_str),
        Err(e) => {
            error!("Could not read database url: {}", e);
            None
        }
    }
}

