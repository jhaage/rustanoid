#!/bin/bash

# First build the project
./build.sh

# Move to the web directory
cd web

# Define server port
PORT=8080

echo "Starting HTTP server at http://localhost:$PORT/"
echo "Press Ctrl+C to stop the server when you're done."
echo "Check your browser console for any JavaScript errors."

# Start a Python HTTP server (most systems have Python installed)
python3 -m http.server $PORT || python -m http.server $PORT