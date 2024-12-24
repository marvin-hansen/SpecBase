use lib_specbase::{SpecBase, Specfile};
use tempfile::tempdir;
use std::{env, fs};

#[test]
fn test_specbase_crud_operations() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join(".config").join("specbase");
    fs::create_dir_all(&config_path).unwrap();
    
    // Set the HOME environment variable to our temp directory
    env::set_var("HOME", temp_dir.path());
    
    // Initialize database
    let spec_db = SpecBase::init().unwrap();
    
    // Test create
    let test_spec = Specfile {
        id: None,
        name: "Test Spec".to_string(),
        description: "Test Description".to_string(),
        content: "Test Content".to_string(),
    };
    
    let id = spec_db.create_specfile(&test_spec).unwrap();
    assert!(id > 0);
    
    // Test read
    let retrieved_spec = spec_db.read_specfile(id).unwrap();
    assert_eq!(retrieved_spec.name, test_spec.name);
    assert_eq!(retrieved_spec.description, test_spec.description);
    assert_eq!(retrieved_spec.content, test_spec.content);
    
    // Test update
    let updated_spec = Specfile {
        id: Some(id),
        name: "Updated Name".to_string(),
        description: "Updated Description".to_string(),
        content: "Updated Content".to_string(),
    };
    
    spec_db.update_specfile(id, &updated_spec).unwrap();
    let retrieved_updated = spec_db.read_specfile(id).unwrap();
    assert_eq!(retrieved_updated.name, updated_spec.name);
    
    // Test list
    let specs = spec_db.list_specfiles().unwrap();
    assert_eq!(specs.len(), 1);
    
    // Test query
    let query_results = spec_db.query_specfiles("Updated").unwrap();
    assert_eq!(query_results.len(), 1);
    assert_eq!(query_results[0].name, updated_spec.name);
    
    // Test delete
    spec_db.delete_specfile(id).unwrap();
    assert!(spec_db.read_specfile(id).is_err());
}

#[test]
fn test_specfile_not_found() {
    let temp_dir = tempdir().unwrap();
    env::set_var("HOME", temp_dir.path());
    
    let spec_db = SpecBase::init().unwrap();
    assert!(spec_db.read_specfile(999).is_err());
    assert!(spec_db.delete_specfile(999).is_err());
}
