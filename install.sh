#!/bin/bash

set -e

PROJECT_DIR="$(pwd)"

echo "Starting build in $PROJECT_DIR..."

if ! command -v cargo &> /dev/null
then
    echo "Error: cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Are you in the Rust project root?"
    exit 1
fi

cargo build --release

echo "✅ Successfully built the project!"

