# Rustanoid

This is a game written in Rust as a learning project and utilizes the *macroquad* game library. The game can be ran locally with `cargo run` or alternatively you can deploy to a local webserver as WASM (web assembly).

## Build & Deploy locally

Build with `cargo build`.

Run with `cargo run`.

## Build & Deploy locally as WASM

The following scripts build the project for a WASM release:

### Linux/MacOS

**build.sh** - builds and then copies index.html, game resources and the rustanoid.wasm to the *web* directory.

**serve.sh** - changes to the *web* directory and starts a local web server on port 8080 (ex: localhost:8080 )

### Windows

**build.bat** - builds and then copies index.html, game resources and the rustanoid.wasm to the *web* directory.

**serve.bat** - changes to the *web* directory and starts a local web server on port 8080 (ex: localhost:8080 )

### Requirements

The serve scripts require Python to be installed on your system. Most systems have Python pre-installed, but if needed:
- Install Python from [python.org](https://python.org)
- Ensure it's added to your system PATH

## Deployment to Next.js (other any other) website project

This project includes scripts to deploy the game to another website project directory. It simply copies the necessary rustanoid.wasm and game resources to a destination directory on your system that you can deploy from.

>Note: Your next.js (or other) website will need to have a page that loads the rustanoid.wasm. You can review the *index.html* file in this project for a simple example.

### Environment Setup

1. Copy the `.env.example` file to `.env`:
   ```
   cp .env.example .env
   ```

2. Edit the `.env` file to set your specific website project path:
   ```
   WEBSITE_PROJECT_PATH=../mysite/public/games/rustanoid
   ```

3. To deploy, run one of the following commands:

   On Linux/macOS:
   ```
   ./deploy-to-nextjs.sh
   ```

   On Windows:
   ```
   deploy-to-nextjs.bat
   ```

The deployment scripts will:
- Build the game
- Create necessary directories in your Next.js project
- Copy the required game files to the Next.js public directory