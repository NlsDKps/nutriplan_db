use std::time::Duration;
use diesel::{
    SqliteConnection,
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection}
};
use dotenv::dotenv;
use log::error;
use std::env;

pub mod ingredient;
pub mod ingredient_macro;
pub mod meal;
pub mod meal_ingredient;
pub mod recipe;
pub mod recipe_ingredient;

pub type ConnMgrPool = PooledConnection<ConnectionManager<SqliteConnection>>;

/**
 * Controller entity to satisfy basic database requirements. The requirements are **C**reate,
 * **R**ead, **U**pdate and **D**elete (CRUD).
 *
 * # TODO
 * * Each method implemented in a trait is a clone from the other implementations, maybe we can
 *   implement a inner method in the CRUDController, which handles the actual db-calls. The
 *   implementation would only set the right parameters (table to access)
 */
trait CRUDController {

    type NewItem;
    type Item;

    /**
     * Create a single item
     */
    fn create(conn_mgr: &ConnMgrPool, new_item: &Self::NewItem) -> bool;

    /**
     * Read item from database by its unique id
     */
    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<Self::Item>;

    /**
     * Change a item characteristics.
     */
    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: Self::Item) -> bool;

    /**
     * Delete an item from database
     */
    fn delete(conn_mgr: &ConnMgrPool, item_id: i32) -> bool;

    /**
     * Check if a item with the specific id exists
     */
    fn check(conn_mgr: &ConnMgrPool, item_id: i32) -> bool {
        match Self::read(conn_mgr, item_id) {
            Some(_) => true,
            None => false
        }
    }
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
pub fn connect_database(database_url: &str)
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

