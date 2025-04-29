@echo off
REM Windows equivalent of serve.sh

echo Building project for WASM...
call build.bat

echo Navigating to web directory...
cd web

set PORT=8080
echo Starting HTTP server at http://localhost:%PORT%/
echo Opening your browser...
start http://localhost:%PORT%/

echo Press Ctrl+C to stop the server when you're done.
echo Check your browser console for any JavaScript errors.

REM Try python3 first, fall back to python if needed
python3 -m http.server %PORT% 2>nul || python -m http.server %PORT%