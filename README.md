# SpecBase

A command-line tool for managing specification files in a structured and easy-to-use way.

## Features

- Store and manage specification files in a SQLite database
- Full-text search capabilities
- Simple command-line interface
- File-based or direct content input
- CRUD operations for specifications

## Installation

### From Source (Linux and macOS)

1. Clone the repository:
```bash
git clone https://github.com/marvin-hansen/specbase.git
cd specbase
```

2. Run the install script:
```bash
./install.sh
```

This will:
- Build the project in release mode
- Create ~/bin directory if it doesn't exist
- Copy the binary to ~/bin/spec
- Make it executable

3. Add ~/bin to your PATH if not already added:
```bash
# Add this to your ~/.bashrc, ~/.zshrc, or equivalent
export PATH="$HOME/bin:$PATH"
```

4. Reload your shell or run:
```bash
source ~/.bashrc  # or ~/.zshrc
```

5. Use spec i.e.

spec --help

## Usage

Initialize a new spec database (stored in ~/.config/specbase/specbase.db):
```bash
spec init
```
If the database already exists, you'll be asked if you want to override it.

Add a new specfile:
```bash
# Add with direct content
spec add --name "specfile1" --description "This is a specfile for a package" --content "This is the content of the specfile"

# Add from file
spec add --name "specfile1" --description "This is a specfile for a package" --file "path/to/file"
```
The command will print the ID of the newly added specfile.

Read a specfile:
```bash
spec get 1
```
This will print the content of the specfile.

Update a specfile:
```bash
spec update --id 1 --name "specfile1" --description "This is a specfile for a package" --content "This is the new content of the specfile"
```
The command will print:
- "ok" if the update was successful
- "error" if the update failed
- "specfile does not exist" if the specfile is not found

Delete a specfile:
```bash
spec delete 1
```
The command will print:
- "ok" if successful
- "specfile does not exist" if the specfile is not found

List all specfiles:
```bash
spec list
```
The command will print:
- A list of all specfiles with their IDs, names, and descriptions
- "ok" if successful
- "specfile does not exist" if no specfiles are found

Search specfiles:
```bash
spec query "new content"
```
This will perform a full-text search across all specfiles and display matching results.

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
cargo run --example basic_usage
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.