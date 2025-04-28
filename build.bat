@echo off
REM Windows equivalent of build.sh

REM Build for wasm32
cargo build --target wasm32-unknown-unknown --release
if %ERRORLEVEL% NEQ 0 (
    echo Error building project
    exit /b %ERRORLEVEL%
)

REM Create web directory structure if it doesn't exist
if not exist web\rustanoid\res mkdir web\rustanoid\res

REM Copy resources
xcopy /s /y /q res\* web\rustanoid\res\

REM Copy index.html to web directory
copy /y index.html web\

REM Copy the wasm binary
copy /y target\wasm32-unknown-unknown\release\rustanoid.wasm web\

echo Build complete! The game is ready in the web\ directory