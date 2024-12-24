//! SpecBase CLI
//! 
//! A command-line tool for managing specification files in a structured way.
//! Uses SQLite as a backend database to store and query specifications.
//! 
//! # Usage
//! 
//! Initialize a new database:
//! ```bash
//! spec init
//! ```
//! 
//! Add a new specification:
//! ```bash
//! spec add --name "My Spec" --description "Description" --content "# Content"
//! # or from file
//! spec add --name "My Spec" --description "Description" --file path/to/spec.md
//! ```

use clap::{Parser, Subcommand};
use lib_specbase::{SpecBase, Specfile};
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

/// Version string from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Command-line interface for SpecBase
#[derive(Parser)]
#[command(name = "spec")]
#[command(about = "SpecBase CLI - A tool to manage specification files")]
#[command(version = VERSION)]
struct Cli {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the SpecBase CLI
#[derive(Subcommand)]
enum Commands {
    /// Initialize a new spec database in ~/.config/specbase/
    Init,
    
    /// Add a new specfile to the database
    Add {
        /// Name of the specification
        #[arg(long)]
        name: String,
        /// Brief description of the specification
        #[arg(long)]
        description: String,
        /// Content of the specification in markdown format
        #[arg(long)]
        content: Option<String>,
        /// Path to a file containing the specification content
        #[arg(long)]
        file: Option<PathBuf>,
    },
    
    /// Retrieve a specfile by its ID
    Get {
        /// ID of the specfile to retrieve
        id: i64,
    },
    
    /// Update an existing specfile
    Update {
        /// ID of the specfile to update
        #[arg(long)]
        id: i64,
        /// New name for the specification
        #[arg(long)]
        name: String,
        /// New description for the specification
        #[arg(long)]
        description: String,
        /// New content for the specification
        #[arg(long)]
        content: String,
    },
    
    /// Delete a specfile by its ID
    Delete {
        /// ID of the specfile to delete
        id: i64,
    },
    
    /// List all specfiles in the database
    List,
    
    /// Search for specfiles using fulltext search
    Query {
        /// Search term to look for in names, descriptions, and content
        query: String,
    },
}

/// Main entry point for the SpecBase CLI
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init => {
            let config_dir = dirs::config_dir()
                .context("Failed to get config directory")?
                .join("specbase");
            let db_path = config_dir.join("specbase.db");
            
            if db_path.exists() {
                println!("Database already exists at {:?}. Do you want to override it? [y/N]", db_path);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Operation aborted");
                    return Ok(());
                }
            }
            
            SpecBase::init()?;
            println!("Initialized new spec database at {:?}", db_path);
        }
        
        Commands::Add { name, description, content, file } => {
            let content = if let Some(file_path) = file {
                fs::read_to_string(file_path)?
            } else {
                content.context("Either --content or --file must be provided")?
            };
            
            let specfile = Specfile {
                id: None,
                name,
                description,
                content,
            };
            
            let spec_db = SpecBase::init()?;
            let id = spec_db.create_specfile(&specfile)?;
            println!("Added new specfile with ID: {}", id);
        }
        
        Commands::Get { id } => {
            let spec_db = SpecBase::init()?;
            match spec_db.read_specfile(id) {
                Ok(specfile) => println!("{}", specfile.content),
                Err(_) => println!("specfile does not exist"),
            }
        }
        
        Commands::Update { id, name, description, content } => {
            let specfile = Specfile {
                id: Some(id),
                name,
                description,
                content,
            };
            
            let spec_db = SpecBase::init()?;
            match spec_db.update_specfile(id, &specfile) {
                Ok(_) => println!("ok"),
                Err(e) => {
                    if e.to_string().contains("not found") {
                        println!("specfile does not exist");
                    } else {
                        println!("error");
                    }
                }
            }
        }
        
        Commands::Delete { id } => {
            let spec_db = SpecBase::init()?;
            match spec_db.delete_specfile(id) {
                Ok(_) => println!("ok"),
                Err(_) => println!("specfile does not exist"),
            }
        }
        
        Commands::List => {
            let spec_db = SpecBase::init()?;
            match spec_db.list_specfiles() {
                Ok(specfiles) => {
                    for specfile in specfiles {
                        println!("ID: {}", specfile.id.unwrap());
                        println!("Name: {}", specfile.name);
                        println!("Description: {}", specfile.description);
                        println!("---");
                    }
                    println!("ok");
                }
                Err(_) => println!("specfile does not exist"),
            }
        }
        
        Commands::Query { query } => {
            let spec_db = SpecBase::init()?;
            let specfiles = spec_db.query_specfiles(&query)?;
            for specfile in specfiles {
                println!("ID: {}", specfile.id.unwrap());
                println!("Name: {}", specfile.name);
                println!("Description: {}", specfile.description);
                println!("---");
            }
        }
    }
    
    Ok(())
}
