#!/bin/bash
set -e

# Build for wasm32
cargo build --target wasm32-unknown-unknown --release

# Create web/rustanoid/res directory if it doesn't exist
mkdir -p web/rustanoid/res

# Copy resources from the new structure
cp -r res/* web/rustanoid/res

# Copy index.html to web directory
cp index.html web/

# Copy the wasm binary
cp target/wasm32-unknown-unknown/release/rustanoid.wasm web/

echo "Build complete! The game is ready in the web/ directory"