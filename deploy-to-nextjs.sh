#!/bin/bash
# Replace with your Next.js project path
NEXTJS_PROJECT_PATH="../jeremyhaage/nextjs-website"

# Build the game
./build.sh

# Create directory if it doesn't exist
mkdir -p "$NEXTJS_PROJECT_PATH/public/games/rustanoid"

# Copy game files
cp -r web/rustanoid.wasm "$NEXTJS_PROJECT_PATH/public/games/rustanoid/"
cp -r web/rustanoid/res "$NEXTJS_PROJECT_PATH/public/games/rustanoid/"