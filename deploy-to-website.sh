#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
else
  echo "Error: .env file not found. Please create one from .env.example."
  echo "Example: cp .env.example .env"
  exit 1
fi

# Build the game
./build.sh

# Create directory if it doesn't exist
mkdir -p "$WEBSITE_PROJECT_PATH/public/games/rustanoid"

# Copy game files
cp -r web/rustanoid.wasm "$WEBSITE_PROJECT_PATH/public/games/rustanoid/"
cp -r web/rustanoid/res "$WEBSITE_PROJECT_PATH/public/games/rustanoid/"