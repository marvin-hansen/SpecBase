use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Specfile not found with ID: {0}")]
    SpecfileNotFound(i64),
    #[error("Failed to create config directory")]
    ConfigDirError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specfile {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub content: String,
}

pub struct SpecBase {
    conn: Connection,
}

impl SpecBase {
    pub fn init() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or(SpecError::ConfigDirError)?
            .join("specbase");
        std::fs::create_dir_all(&config_dir)?;
        
        let db_path = config_dir.join("specbase.db");
        let conn = Connection::open(&db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS specfiles (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                content TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn create_specfile(&self, specfile: &Specfile) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO specfiles (name, description, content) VALUES (?1, ?2, ?3)",
            params![specfile.name, specfile.description, specfile.content],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn read_specfile(&self, id: i64) -> Result<Specfile> {
        let specfile = self.conn.query_row(
            "SELECT id, name, description, content FROM specfiles WHERE id = ?1",
            params![id],
            |row| {
                Ok(Specfile {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    description: row.get(2)?,
                    content: row.get(3)?,
                })
            },
        ).map_err(|_| SpecError::SpecfileNotFound(id))?;
        Ok(specfile)
    }

    pub fn update_specfile(&self, id: i64, specfile: &Specfile) -> Result<()> {
        let rows_affected = self.conn.execute(
            "UPDATE specfiles SET name = ?1, description = ?2, content = ?3 WHERE id = ?4",
            params![specfile.name, specfile.description, specfile.content, id],
        )?;
        
        if rows_affected == 0 {
            return Err(SpecError::SpecfileNotFound(id).into());
        }
        Ok(())
    }

    pub fn delete_specfile(&self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM specfiles WHERE id = ?1",
            params![id],
        )?;
        
        if rows_affected == 0 {
            return Err(SpecError::SpecfileNotFound(id).into());
        }
        Ok(())
    }

    pub fn list_specfiles(&self) -> Result<Vec<Specfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, content FROM specfiles"
        )?;
        
        let specfiles = stmt.query_map([], |row| {
            Ok(Specfile {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(specfiles)
    }

    pub fn query_specfiles(&self, query: &str) -> Result<Vec<Specfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, content FROM specfiles 
             WHERE name LIKE ?1 OR description LIKE ?1 OR content LIKE ?1"
        )?;
        
        let search_pattern = format!("%{}%", query);
        let specfiles = stmt.query_map(params![search_pattern], |row| {
            Ok(Specfile {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(specfiles)
    }
}
