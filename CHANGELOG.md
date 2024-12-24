# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-12-24

### Changed
- Updated CLI interface for improved usability:
  - Simplified command syntax for `get` and `delete` (removed --id flag)
  - Simplified `query` command syntax (removed --query flag)
  - Standardized output messages across all commands
  - Added clear success/failure indicators ("ok"/"error")
  - Improved error messages for not found cases
- Enhanced database initialization:
  - Added confirmation prompt for database override
  - Clarified database location (~/.config/specbase/specbase.db)
- Updated documentation to reflect new CLI usage
- Added detailed comments to example code

## [0.1.0] - 2024-01-01

### Added
- Initial release of SpecBase
- Core functionality for managing specification files
- SQLite database backend
- Command-line interface with the following commands:
  - `init`: Initialize a new spec database
  - `add`: Add a new specfile
  - `get`: Retrieve a specfile by ID
  - `update`: Update an existing specfile
  - `delete`: Delete a specfile
  - `list`: List all specfiles
  - `query`: Search specfiles using fulltext search
- Support for both file-based and direct content input
- Basic error handling and user feedback
- Comprehensive test suite
- Example code demonstrating basic usage
