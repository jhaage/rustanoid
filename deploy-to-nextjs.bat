@echo off
REM Windows equivalent of deploy-to-nextjs.sh

REM Replace with your Next.js project path
set NEXTJS_PROJECT_PATH=..\jeremyhaage\nextjs-website

REM Build the game
call build.bat

REM Create directory if it doesn't exist
if not exist "%NEXTJS_PROJECT_PATH%\public\games\rustanoid" mkdir "%NEXTJS_PROJECT_PATH%\public\games\rustanoid"

REM Copy game files
copy /y web\rustanoid.wasm "%NEXTJS_PROJECT_PATH%\public\games\rustanoid\"
xcopy /s /y /q web\rustanoid\res "%NEXTJS_PROJECT_PATH%\public\games\rustanoid\res\"

echo Deployment to Next.js project complete!