# Rustanoid

This is a game written in Rust as a learning project and utilizes the *macroquad* game library. The game can be ran locally with `cargo run` or alternatively you can deploy to a local webserver as WASM (web assembly).

## Requirements

### All Platforms
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

### Linux (Ubuntu/Debian)
For building the native version:
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

For building the WASM version:
```bash
rustup target add wasm32-unknown-unknown
```

### macOS
```bash
brew install pkg-config
```

### Windows
- Visual Studio C++ build tools (comes with Visual Studio or can be installed separately)
- For MinGW users: `gcc` and associated build tools

For building the WASM version (all platforms):
```bash
rustup target add wasm32-unknown-unknown
```

### Python Requirements
The serve scripts require Python to be installed on your system:
- Install Python from [python.org](https://python.org)
- Ensure it's added to your system PATH

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

## Deployment to Next.js (other any other) website project

This project includes scripts to deploy the game to another website project directory. It simply copies the necessary rustanoid.wasm and game resources to a destination directory on your system that you can deploy from.

>Note: Your next.js (or other) website will need to have a page that loads the rustanoid.wasm. You can review the *index.html* file in this project for a simple example.

### Environment Setup

1. Copy the `.env.example` file to `.env`:
   ```
   cp .env.example .env   # On Linux/macOS
   copy .env.example .env # On Windows
   ```

2. Edit the `.env` file to set your specific website project path:
   ```
   WEBSITE_PROJECT_PATH=../mywebsite
   ```

3. To deploy, run one of the following commands:

   On Linux/macOS:
   ```
   ./deploy-to-website.sh
   ```

   On Windows:
   ```
   .\deploy-to-website.bat
   ```

The deployment scripts will:
- Build the game
- Create necessary /public/games/rustanoid directories within your WEBSITE_PROJECT_PATH directory
- Copy the required game files to the WEBSITE_PROJECT_PATH/public/games/rustanoid directory