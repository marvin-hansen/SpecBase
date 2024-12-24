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

### From Cargo (All Platforms)

```bash
cargo install specbase
```

## Usage

Initialize a new spec database:
```bash
spec init
```

Add a new specfile:
```bash
# Add from content
spec add --name "My Spec" --description "A description" --content "# My Specification..."

# Add from file
spec add --name "My Spec" --description "A description" --file path/to/spec.md
```

Read a specfile:
```bash
spec get --id 1
```

Update a specfile:
```bash
spec update --id 1 --name "Updated Name" --description "Updated description" --content "Updated content"
```

Delete a specfile:
```bash
spec delete --id 1
```

List all specfiles:
```bash
spec list
```

Search specfiles:
```bash
spec query --query "search term"
```

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