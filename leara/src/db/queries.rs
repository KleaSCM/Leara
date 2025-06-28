use rusqlite::{Connection, Result as SqliteResult};
use crate::models::{ChatMessage, Memory};
use chrono::{DateTime, Utc, FixedOffset};
use uuid::Uuid;

pub fn store_message(conn: &Connection, message: &ChatMessage) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO messages (id, conversation_id, content, sender, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &message.id.to_string(),
            &message.conversation_id.map(|id| id.to_string()),
            &message.content,
            &format!("{:?}", message.sender),
            &message.timestamp.to_rfc3339(),
        ),
    )?;
    Ok(())
}

pub fn store_memory(conn: &Connection, memory: &Memory) -> SqliteResult<()> {
    conn.execute(
        "INSERT OR REPLACE INTO memory (id, key, value, category, created_at, updated_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &memory.id.to_string(),
            &memory.key,
            &memory.value,
            &memory.category,
            &memory.created_at.to_rfc3339(),
            &memory.updated_at.to_rfc3339(),
            &memory.expires_at.map(|dt| dt.to_rfc3339()),
        ),
    )?;
    Ok(())
}

pub fn get_memory_by_key(conn: &Connection, key: &str) -> SqliteResult<Option<Memory>> {
    let mut stmt = conn.prepare(
        "SELECT id, key, value, category, created_at, updated_at, expires_at FROM memory WHERE key = ?1"
    )?;
    
    let mut rows = stmt.query([key])?;
    if let Some(row) = rows.next()? {
        let fallback = DateTime::<FixedOffset>::from(Utc::now());
        let memory = Memory {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_else(|_| Uuid::new_v4()),
            key: row.get(1)?,
            value: row.get(2)?,
            category: row.get(3)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .unwrap_or(fallback)
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .unwrap_or(fallback)
                .with_timezone(&Utc),
            expires_at: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
        };
        Ok(Some(memory))
    } else {
        Ok(None)
    }
} 