use anyhow::Result;
use lib_specbase::{SpecBase, Specfile};

fn main() -> Result<()> {
    // Initialize the database
    let spec_db = SpecBase::init()?;
    println!("Initialized SpecBase database");

    // Create a new specfile
    let spec = Specfile {
        id: None,
        name: "Example Spec".to_string(),
        description: "An example specification file".to_string(),
        content: "# Example Specification\n\nThis is an example specification.".to_string(),
    };

    let id = spec_db.create_specfile(&spec)?;
    println!("Created specfile with ID: {}", id);

    // Read the specfile
    let retrieved = spec_db.read_specfile(id)?;
    println!("\nRetrieved specfile:");
    println!("Name: {}", retrieved.name);
    println!("Description: {}", retrieved.description);
    println!("Content:\n{}", retrieved.content);

    // Update the specfile
    let updated = Specfile {
        id: Some(id),
        name: "Updated Example".to_string(),
        description: "Updated description".to_string(),
        content: "# Updated Specification\n\nThis specification has been updated.".to_string(),
    };

    spec_db.update_specfile(id, &updated)?;
    println!("\nUpdated specfile successfully");

    // List all specfiles
    println!("\nListing all specfiles:");
    for spec in spec_db.list_specfiles()? {
        println!("ID: {}", spec.id.unwrap());
        println!("Name: {}", spec.name);
        println!("Description: {}", spec.description);
        println!("---");
    }

    // Query specfiles
    println!("\nQuerying for 'Updated':");
    for spec in spec_db.query_specfiles("Updated")? {
        println!("Found: {} (ID: {})", spec.name, spec.id.unwrap());
    }

    // Delete the specfile
    spec_db.delete_specfile(id)?;
    println!("\nDeleted specfile with ID: {}", id);

    Ok(())
}
