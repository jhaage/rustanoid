@echo off
REM Windows equivalent of serve.sh

REM First build the project
call build.bat

REM Move to the web directory
cd web

REM Start a Python HTTP server
python -m http.server 8080
REM If the above fails, try with python3
if %ERRORLEVEL% NEQ 0 (
    python3 -m http.server 8080
)