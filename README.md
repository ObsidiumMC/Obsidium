<div align="center">

# Obsidium

A high-performance Minecraft server implementation written in Rust.

<a target="_blank" href="https://discord.gg/XyKfC4WjUw"><img src="https://dcbadge.limes.pink/api/server/XyKfC4WjUw" alt="" /></a>

</div>

## Overview

Obsidium is a modern Minecraft Java Edition server built from the ground up in Rust, focusing on performance, modularity, and maintainability. It implements the Minecraft protocol version 770 (Minecraft 1.21.5).

## Current Status

⚠️ **Early Development** - This project is in active development and not yet ready for production use.

### What Currently Works

- ✅ Network layer with connection management
- ✅ Basic protocol implementation (handshaking, status, login)
- ✅ Server list ping functionality
- ✅ Player authentication (offline mode)
- ✅ Packet compression support
- ✅ Player connection/disconnection handling
- ✅ Configurable server settings
- ✅ Beautiful colored logging

### What's Not Implemented Yet

- ❌ World initialization - Players get stuck on "Joining world..." and timeout
- ❌ Chunk streaming to players (chunks exist but aren't sent)
- ❌ Player spawn and positioning packets
- ❌ Most gameplay packets (movement, block interactions, etc.)
- ❌ Entity spawning and management
- ❌ Inventory system
- ❌ World persistence
- ❌ Online mode (Mojang authentication)
- ❌ Command system
- ❌ Plugin/mod support

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building and Running

```bash
# Clone the repository
git clone https://github.com/OmarAfet/obsidium.git
cd obsidium

# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start on `localhost:25565` by default.

### Configuration

You can customize server settings by modifying the configuration in `src/main.rs` or by setting environment variables:

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable timestamps in logs (disabled by default)
RUST_LOG_TIME=1 cargo run

# Combine both options
RUST_LOG=debug RUST_LOG_TIME=1 cargo run

# The server currently uses hardcoded configuration
# Full configuration file support is planned
```

#### Logging Options

The server uses a custom logger with colored output:

- **Default format**: `[INFO] message` (no timestamps)
- **With timestamps**: `HH:MM:SS.mmm [INFO] message`
- **Log levels**: `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`

Environment variables:
- `RUST_LOG`: Set log level (`error`, `warn`, `info`, `debug`, `trace`)
- `RUST_LOG_TIME`: Enable timestamps (`1` or `true` to enable)

## Architecture

```
src/
├── config/          # Server configuration management
├── error.rs         # Error handling and types
├── game/            # Game logic
│   ├── entity/      # Entity system
│   ├── player.rs    # Player management
│   └── world/       # World and chunk management
├── logger.rs        # Custom logging implementation
├── network/         # Low-level networking
│   ├── connection.rs # Connection handling
│   └── listener.rs   # Server listener
├── protocol/        # Minecraft protocol implementation
│   ├── packets/     # Packet definitions by state
│   ├── types.rs     # Protocol data types
│   └── compression.rs # Packet compression
└── server/          # Main server implementation
```

## Testing

You can test the server using any Minecraft 1.21.5 client:

1. Start the server with `cargo run`
2. In Minecraft, add a server with address `localhost:25565`
3. The server should appear in your server list and show as online
4. **Note**: Attempting to join will currently result in a timeout at "Joining world..." - world initialization packets are not yet implemented

## Contributing

This project is in early development and welcomes contributions! Here are some ways you can help:

- **Implement missing packets**: Many play-state packets need implementation
- **Add gameplay features**: Block interactions, entity movement, etc.
- **Improve world generation**: Currently only generates flat worlds
- **Add tests**: More comprehensive test coverage is needed
- **Documentation**: Help improve documentation and examples

### Development Setup

```bash
# Clone the repository
git clone https://github.com/OmarAfet/obsidium.git
cd obsidium

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Run with debug logging and timestamps
RUST_LOG=debug RUST_LOG_TIME=1 cargo run

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Roadmap

### Short Term (v0.1)
- [ ] World initialization packets - Allow players to actually join the server
- [ ] Chunk streaming to players
- [ ] Basic player movement synchronization
- [ ] Block breaking and placing
- [ ] Basic inventory system

### Medium Term (v0.2)
- [ ] World persistence and loading
- [ ] Online mode with Mojang authentication
- [ ] Entity AI and spawning
- [ ] Basic command system

### Long Term (v1.0)
- [ ] Plugin/mod API
- [ ] Performance optimizations
- [ ] Full feature parity with vanilla
- [ ] Administration tools

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.