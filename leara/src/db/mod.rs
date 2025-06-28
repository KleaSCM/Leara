use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use tracing::info;

pub mod migrations;
pub mod queries;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

pub async fn init_database(path: &str) -> anyhow::Result<()> {
    // Ensure data directory exists
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut db = Database::new(path)?;
    
    // Run migrations
    migrations::run_migrations(&mut db)?;
    
    info!("Database init successfully");
    Ok(())
} 