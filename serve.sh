#!/bin/bash

# First build the project
./build.sh

# Move to the web directory
cd web

# Start a Python HTTP server (most systems have Python installed)
python3 -m http.server 8080