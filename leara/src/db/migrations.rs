use rusqlite::Result as SqliteResult;
use crate::db::Database;
use tracing::info;

pub fn run_migrations(db: &mut Database) -> SqliteResult<()> {
    info!("Running database migrations...");

    // Create conversations table
    db.get_connection_mut().execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            message_count INTEGER DEFAULT 0
        )",
        [],
    )?;

    // Create messages table
    db.get_connection_mut().execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT,
            content TEXT NOT NULL,
            sender TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations (id)
        )",
        [],
    )?;

    // Create memory table
    db.get_connection_mut().execute(
        "CREATE TABLE IF NOT EXISTS memory (
            id TEXT PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            category TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            expires_at TEXT
        )",
        [],
    )?;

// TODO extend to goals and desires 

    // Create indexes
    db.get_connection_mut().execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages (conversation_id)",
        [],
    )?;

    db.get_connection_mut().execute(
        "CREATE INDEX IF NOT EXISTS idx_memory_key ON memory (key)",
        [],
    )?;

    db.get_connection_mut().execute(
        "CREATE INDEX IF NOT EXISTS idx_memory_category ON memory (category)",
        [],
    )?;

    info!("Database migrations completed successfully");
    Ok(())
} 