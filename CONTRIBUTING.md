# Contributing to Obsidium

First off, thank you for considering contributing to Obsidium. Every contribution is appreciated and helps move the project forward.

## Where to Start

As noted in our [README](README.md#current-state-under-development), the server is in the early stages of development.

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please open an issue on our [GitHub Issues](https://github.com/ObsidiumMC/Obsidium/issues) page.

When filing a bug report, please include:
- A clear and descriptive title.
- Steps to reproduce the bug.
- The Obsidium version or commit hash you are using.
- Any relevant logs or error messages.

### Suggesting Enhancements

If you have an idea for a new feature or an improvement, feel free to open an issue to discuss it. This allows us to coordinate efforts and ensure the suggestion aligns with the project's goals.

### Pull Requests

We welcome pull requests for bug fixes, new features, and improvements.

#### Development Setup

1.  **Install Rust:** If you don't have it, get it from [rustup.rs](https://rustup.rs/).
2.  **Install Required Components:** Our CI system uses `rustfmt` for formatting and `clippy` for linting. You should install them locally:
    ```sh
    rustup component add rustfmt clippy
    ```
3.  **Fork and Clone:** Fork the repository on GitHub, then clone your fork:
    ```sh
    git clone https://github.com/YOUR_USERNAME/Obsidium.git
    cd Obsidium
    ```

#### Pull Request Process

1.  Create a new branch for your feature or fix: `git checkout -b your-feature-name`.
2.  Make your changes.
3.  Before committing, ensure your code is well-formatted and free of lints:
    ```sh
    # Format your code
    cargo fmt --all

    # Run Clippy to catch common mistakes
    cargo clippy --all-targets --all-features -- -D warnings
    ```
4.  Ensure all tests pass: `cargo test --all-features`.
5.  Commit your changes with a clear and descriptive message.
6.  Push your branch to your fork and open a pull request to the `master` branch of the main Obsidium repository.

## Code Style

-   We follow the standard Rust style guidelines. Use `cargo fmt` to automatically format your code.
-   We use `clippy` to enforce a higher level of code quality. Please address any warnings reported by `clippy`.
-   Write simple, logical, and self-documenting code. Add comments for complex or non-obvious logic.

Thank you for your contribution!