@echo off
REM Windows equivalent of serve.sh

echo Building project for WASM...
call build.bat

echo Navigating to web directory...
cd web

echo Opening browser at http://localhost:8080/
start http://localhost:8080/

echo Starting HTTP server on port 8080...
echo Press Ctrl+C to stop the server when you're done.
echo Check your browser console for any JavaScript errors.
python -m http.server 8080
REM If the above fails, try with python3
if %ERRORLEVEL% NEQ 0 (
    python3 -m http.server 8080
)