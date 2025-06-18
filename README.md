<div align="center">

# Obsidium

**A high-performance Minecraft server written in Rust.**

<p>
    <a target="_blank" href="https://github.com/ObsidiumMC/Obsidium/actions/workflows/checks.yml"><img src="https://github.com/ObsidiumMC/Obsidium/actions/workflows/checks.yml/badge.svg" alt="Code Quality Checks" /></a>
    <a target="_blank" href="https://www.minecraft.net"><img src="https://img.shields.io/badge/Minecraft-1.21.6-brightgreen.svg" alt="Minecraft Version" /></a>
    <a target="_blank" href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.87.0-blue.svg" alt="Rust Version" /></a>
    <a target="_blank" href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
    <br />
    <br />
    <a target="_blank" href="https://discord.gg/XyKfC4WjUw"><img src="https://dcbadge.limes.pink/api/server/XyKfC4WjUw" alt="Discord Server" /></a>
</p>

</div>

## Current State

Obsidium is currently under heavy development and is not yet ready for gameplay.

**What works:**
-   The server starts and listens for connections.
-   Clients can see the server in their multiplayer list (server list ping).
-   Players can successfully connect and log in, passing the handshaking, login, and configuration states.

**What does not work:**
-   Players cannot spawn in the world yet.

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

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more details. For real-time discussion with developers, join our [Discord Server](https://discord.gg/XyKfC4WjUw).

## License

This project is licensed under the [MIT License](LICENSE).