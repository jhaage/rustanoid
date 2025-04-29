@echo off
REM Windows equivalent of deploy-to-nextjs.sh

REM Load environment variables from .env file
if exist .env (
    for /f "tokens=*" %%a in (.env) do (
        set "%%a"
    )
) else (
    echo Error: .env file not found. Please create one from .env.example.
    echo Example: copy .env.example .env
    exit /b 1
)

REM Build the game
call build.bat

REM Create directory if it doesn't exist
if not exist "%WEBSITE_PROJECT_PATH%\public\games\rustanoid" mkdir "%WEBSITE_PROJECT_PATH%\public\games\rustanoid"

REM Copy game files
copy /y web\rustanoid.wasm "%WEBSITE_PROJECT_PATH%\public\games\rustanoid\"
xcopy /s /y /q web\rustanoid\res "%WEBSITE_PROJECT_PATH%\public\games\rustanoid\res\"

echo Deployment to Next.js project complete!