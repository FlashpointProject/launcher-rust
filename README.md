# Launcher (Rust)

## Crates
- **flashpoint** - Binary to run the backend independently
- **flashpoint-core** - Provides Flashpoint Service
  - Features:
  - **services** - When enabled it will load info from services.json and run any background services itself
  - **websocket** - Enables access to `.listen()` to consume the Service and start Websocket communication
- **flashpoint-config** - Contains Config, Prefs and Service Info structs
- **flashpoint-database** - Everything related to interacting with the DB, including structs for Game, Tag etc

## Development

**Build + Run Binary** - `cargo run`

**Release Binary** - `cargo build --release`
- If targetting Windows 32bit machines:
- Add toolchain (run once) `rustup target add i686-pc-windows-msvc`
- Add target to build command `cargo build --release --target i686-pc-windows-msvc`
