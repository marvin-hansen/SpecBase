use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when working with SpecBase
#[derive(Error, Debug)]
pub enum SpecError {
    /// Represents errors that occur during database operations
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    /// Indicates that a specfile with the given ID was not found
    #[error("Specfile not found with ID: {0}")]
    SpecfileNotFound(i64),

    /// Indicates that the config directory could not be created
    #[error("Failed to create config directory")]
    ConfigDirError,
}

/// Represents a specification file in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Specfile {
    /// Unique identifier for the specfile. None if not yet saved to database.
    pub id: Option<i64>,
    /// Name of the specification
    pub name: String,
    /// Brief description of the specification
    pub description: String,
    /// Full content of the specification in markdown format
    pub content: String,
}

/// Main struct for interacting with the SpecBase database
pub struct SpecBase {
    conn: Connection,
}

impl SpecBase {
    /// Initializes a new SpecBase instance with a SQLite database
    ///
    /// Creates a new database file at ~/.config/specbase/specbase.db if it doesn't exist.
    /// Also creates the necessary tables for storing specfiles.
    ///
    /// # Returns
    /// * `Ok(SpecBase)` - Successfully initialized database connection
    /// * `Err(Error)` - Failed to create config directory or initialize database
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::SpecBase;
    ///
    /// let spec_db = SpecBase::init().expect("Failed to initialize database");
    /// ```
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
}

impl SpecBase {
    /// Creates a new specfile in the database
    ///
    /// # Arguments
    /// * `specfile` - The specfile to create. The `id` field will be ignored.
    ///
    /// # Returns
    /// * `Ok(i64)` - ID of the newly created specfile
    /// * `Err(Error)` - Failed to create specfile in database
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::{SpecBase, Specfile};
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// let spec = Specfile {
    ///     id: None,
    ///     name: "Example".to_string(),
    ///     description: "An example spec".to_string(),
    ///     content: "# Example\nThis is an example.".to_string(),
    /// };
    ///
    /// let id = spec_db.create_specfile(&spec).expect("Failed to create specfile");
    /// ```
    pub fn create_specfile(&self, specfile: &Specfile) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO specfiles (name, description, content) VALUES (?1, ?2, ?3)",
            params![specfile.name, specfile.description, specfile.content],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Retrieves a specfile from the database by its ID
    ///
    /// # Arguments
    /// * `id` - The ID of the specfile to retrieve
    ///
    /// # Returns
    /// * `Ok(Specfile)` - The requested specfile
    /// * `Err(SpecError::SpecfileNotFound)` - No specfile found with the given ID
    /// * `Err(Error)` - Other database error occurred
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::SpecBase;
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// match spec_db.read_specfile(1) {
    ///     Ok(spec) => println!("Found spec: {}", spec.name),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn read_specfile(&self, id: i64) -> Result<Specfile> {
        let specfile = self
            .conn
            .query_row(
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
            )
            .map_err(|_| SpecError::SpecfileNotFound(id))?;
        Ok(specfile)
    }

    /// Updates an existing specfile in the database
    ///
    /// # Arguments
    /// * `id` - The ID of the specfile to update
    /// * `specfile` - The new specfile data. The `id` field will be ignored.
    ///
    /// # Returns
    /// * `Ok(())` - Successfully updated the specfile
    /// * `Err(SpecError::SpecfileNotFound)` - No specfile found with the given ID
    /// * `Err(Error)` - Other database error occurred
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::{SpecBase, Specfile};
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// let updated_spec = Specfile {
    ///     id: Some(1),
    ///     name: "Updated Example".to_string(),
    ///     description: "Updated description".to_string(),
    ///     content: "# Updated\nThis spec has been updated.".to_string(),
    /// };
    ///
    /// match spec_db.update_specfile(1, &updated_spec) {
    ///     Ok(_) => println!("Successfully updated specfile"),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
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

    /// Deletes a specfile from the database
    ///
    /// # Arguments
    /// * `id` - The ID of the specfile to delete
    ///
    /// # Returns
    /// * `Ok(())` - Successfully deleted the specfile
    /// * `Err(SpecError::SpecfileNotFound)` - No specfile found with the given ID
    /// * `Err(Error)` - Other database error occurred
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::SpecBase;
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// match spec_db.delete_specfile(1) {
    ///     Ok(_) => println!("Successfully deleted specfile"),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn delete_specfile(&self, id: i64) -> Result<()> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM specfiles WHERE id = ?1", params![id])?;

        if rows_affected == 0 {
            return Err(SpecError::SpecfileNotFound(id).into());
        }
        Ok(())
    }
}

impl SpecBase {
    /// Lists all specfiles in the database
    ///
    /// # Returns
    /// * `Ok(Vec<Specfile>)` - List of all specfiles
    /// * `Err(Error)` - Failed to query database
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::SpecBase;
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// match spec_db.list_specfiles() {
    ///     Ok(specs) => {
    ///         for spec in specs {
    ///             println!("Found spec: {} (ID: {})", spec.name, spec.id.unwrap());
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn list_specfiles(&self) -> Result<Vec<Specfile>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, content FROM specfiles")?;

        let specfiles = stmt
            .query_map([], |row| {
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

    /// Searches for specfiles using a fulltext query
    ///
    /// Searches through the name, description, and content of all specfiles
    /// for matches with the given query string. The search is case-insensitive
    /// and uses SQL LIKE with wildcards.
    ///
    /// # Arguments
    /// * `query` - The search term to look for
    ///
    /// # Returns
    /// * `Ok(Vec<Specfile>)` - List of matching specfiles
    /// * `Err(Error)` - Failed to query database
    ///
    /// # Example
    /// ```no_run
    /// use lib_specbase::SpecBase;
    ///
    /// let spec_db = SpecBase::init().unwrap();
    /// match spec_db.query_specfiles("example") {
    ///     Ok(specs) => {
    ///         println!("Found {} matching specs:", specs.len());
    ///         for spec in specs {
    ///             println!("- {} (ID: {})", spec.name, spec.id.unwrap());
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn query_specfiles(&self, query: &str) -> Result<Vec<Specfile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, content FROM specfiles 
             WHERE name LIKE ?1 OR description LIKE ?1 OR content LIKE ?1",
        )?;

        let search_pattern = format!("%{}%", query);
        let specfiles = stmt
            .query_map(params![search_pattern], |row| {
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
