<div align="center">

# Obsidium

**A high-performance, Minecraft server written in Rust.**

[![Code Quality Checks](https://github.com/ObsidiumMC/Obsidium/actions/workflows/checks.yml/badge.svg)](https://github.com/ObsidiumMC/Obsidium/actions/workflows/checks.yml)
[![Minecraft Version](https://img.shields.io/badge/Minecraft-1.21.6-brightgreen.svg)](https://www.minecraft.net)
[![Rust Version](https://img.shields.io/badge/rust-1.87.0-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

## Current State: Under Development

Obsidium is currently under heavy development and is not yet ready for gameplay.

**What works:**
-   The server starts and listens for connections.
-   Clients can see the server in their multiplayer list (server list ping).
-   Players can successfully connect and log in, passing the handshaking, login, and configuration states.

**What does not work:**
-   **Players cannot spawn in the world yet.**

## Getting Started

The easiest way to run Obsidium is to use a pre-built binary from the [Releases](https://github.com/ObsidiumMC/Obsidium/releases) page.

1.  Download the executable for your operating system (Windows, Linux, or macOS).
2.  Run the executable from your terminal.

That's it! The server is now running on your machine at `localhost:25565`.

## Building from Source

If you want to contribute or build the server yourself, you'll need the Rust toolchain.

1.  **Install Rust:** Get it from [rustup.rs](https://rustup.rs/).
2.  **Clone the repository:**
    ```sh
    git clone https://github.com/ObsidiumMC/Obsidium.git
    cd Obsidium
    ```
3.  **Run the server:**
    ```sh
    cargo run
    ```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the [MIT License](LICENSE).