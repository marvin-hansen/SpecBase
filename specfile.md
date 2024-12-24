# Specfile

+++
title = 'Specs for SpecBase CLI'
date = 2024-01-01T12:02:07+08:00
version = 1
+++

## Overview

This is a specification file for the SpecBase CLI.

SpecBase is a command-line tool that allows users to manage their specfiles in a structured and easy-to-use way.
Internally, SpecBase uses a SQLite database to store and query all specfiles.

## Context

Current crate: specbase
Programming language: Rust
Build tool: Cargo

### Internal dependencies:

### External dependencies:
* sqlite3
* clap

### Related files

## Requirements

### Functional Requirements

specfile:
* A specfile is a Markdown file that contains requirement and specification information about a specific software package or library.

**Managing specfiles**
* Add a new specfile
* Read an existing specfile
* Update an existing specfile
* Delete an existing specfile

**Query and list existing specfiles**
* List all specfiles
* Query all specfiles using fulltext

**CLI**
* CLI with standard help format for each command
* Ability to interrupt long run running commands with ctrl-c

**Methods**

0) init

Initialize a new spec database
takes no parameters. creates a new sqlite database and stores it under the name specbase.db 
in the home directory in the folder ~/.config/specbase

All DB access is from and to the DB file in ~/.config/specbase/specbase.db

1) crate_specfile
 
Takes an input specfile, and creates a new specfile the database; returns the ID of the specfile.
 
2) read_specfile
 
Takes as input the ID of a specfile, and returns the fulltext of the specfile.
 
3) update_specfile
 

Takes as input the ID of a specfile, and an input specfile, and updates the existing specfile in the database.

4) delete_specfile


Takes as input the ID of a specfile, and deletes the specfile from the database.

5) list_specfiles

Returns a list of all specfiles in the database.

6) query_specfiles

Takes as input a string, performs fulltext search, and returns a list of all specfiles that match the string.


**CLI Usage Example**

* Initialize a new spec database stored in  ~/.config/specbase/specbase.db
spec init

Check if the file exists, and if so, ask if the user wants to override it or abort the operation?

* Add a new specfile by content 
spec add --name "specfile1" --description "This is a specfile for a package" --content "This is the content of the specfile"

print the ID of the newly added specfile 
 
* Add a new specfile by file
spec add --name "specfile1" --description "This is a specfile for a package" --file "path/to/file"

print the ID of the newly added specfile

* Read an existing specfile
spec get --id 1 
 
print out the content of the specfile

* Update an existing specfile
spec update --id 1 --name "specfile1" --description "This is a specfile for a package" --content "This is the new content of the specfile"
 
print ok, if the update was successful.
Print error, if the update failed.
Print "specfile does not exist" if the specfile is not found

* Delete an existing specfile
spec delete --id 1

* List all specfiles
spec list

print ok, if the update was successful.
Print "specfile does not exist" if the specfile is not found

* Query all specfiles using fulltext
spec query --query "new content"

* show help
spec --help

* show version
spec --version

### Non-Functional Requirements

**Performance**
Optimize the hotpath code for best performance
Only use safe Rust APIs and features for performance optimization.
Minimize or avoid runtime memory allocations by either pre-allocating correctly or setting sizes at compile time.
Use parallelism when possible.

**Reliability**
Error handling and recovery
Proper resource allocation and deallocation
Proper resource cleanup
Prevention of memory leaks

**Security**
Limit scope if internal methods
Minimize usage of external dependencies
Apply security best practices to prevent security vulnerabilities.  

### Tasks

**Build:**
* Implement all requirements stated above as a separated library crate called lib_specbase
* Implement the CLI in the main.rs file
* Build all crates with cargo build

**Test:**
* When the crate builds, proceed with testing lib_specbase. 
* Create a test folder, or of it already exists, add or update test files in the test folder 
* Generate full tests coverage with all tests in dedicated test files in the test folder
* Run all tests using cargo test

**Example:**
* When all tests pass, crate an example folder.
* Add an example file to the example folder showcasing the usage of the CLI
* Add the example to Cargo.toml
* Ensure that the example code builds and runs 

**Document:**
* When all examples build and all tests pass, document all public methods with comprehensive docstring
* Generate or update the Readme.md file
*Generate or update the changelog.md file and document all changes made to the crate with today's date

**Finalize:**
Generate a git commit message summarizing all changes made to the crate and print the commit message to the terminal.