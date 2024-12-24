#!/bin/bash

# Exit on any error
set -e

echo "Building SpecBase in release mode..."
cargo build --release

echo "Creating ~/bin directory if it doesn't exist..."
mkdir -p ~/bin

echo "Copying binary to ~/bin/spec..."
cp target/release/spec ~/bin/spec

echo "Making binary executable..."
chmod +x ~/bin/spec

echo "Testing installation..."
~/bin/spec --version

echo "Installation complete!"
echo "Make sure ~/bin is in your PATH to use 'spec' command globally."
