# Dotsy Project Context

## Project Overview

**Dotsy** is a CLI-based dotfiles manager written in **Rust**. It is designed to be simple, easy to use, and focused solely on managing dotfiles via symlinks, avoiding the complexity of other tools like `stow` or `chezmoi`.

**Status:** Early development (Alpha).
**Architecture:**
-   **Language:** Rust (Edition 2024, v1.92.0)
-   **CLI Framework:** `clap`
-   **Configuration:** TOML
-   **Core Logic:** Symlink management (creation, purging, repair)

## Key Files & Directories

-   `src/`: Source code.
    -   `main.rs`: Entry point.
    -   `cli.rs`: CLI command definitions (Switch, Doctor, Init, List, Purge, Repair).
    -   `commands/`: Implementation of specific CLI commands.
    -   `config/`: Configuration handling (TOML parsing).
-   `mise.toml`: Project tool configuration (Rust version).
-   `mise-tasks/`: Helper scripts for containerized development (`build-container.sh`, `run-container.sh`).
-   `Cargo.toml`: Rust dependencies and package metadata.
-   `schema/dotsy.json`: JSON schema for validation (likely for the TOML config).

## Building and Running

### Prerequisites
-   **Rust:** v1.92.0 (managed via `mise` or `rustup`).
-   **Mise:** Recommended for environment and task management.

### Development Commands

1.  **Build:**
    ```bash
    cargo build
    ```

2.  **Run:**
    ```bash
    cargo run -- <command>
    # Example: cargo run -- list
    ```

3.  **Containerized Environment (Recommended):**
    The project includes `mise` tasks to run Dotsy in a container to avoid accidental data loss on the host system during development.
    ```bash
    # Check available tasks (inferred)
    mise run build-container
    mise run run-container
    ```

## Development Conventions

-   **Formatting:** Follows standard Rust formatting (`rustfmt.toml` is present). Run `cargo fmt` before committing.
-   **Configuration:** Uses TOML for user configuration.
-   **Safety:** Due to the nature of file system operations (symlinking, deletion), testing in a container is highly encouraged.

## CLI Commands (`src/cli.rs`)

-   `init`: Initialize Dotsy.
-   `list`: List available configs.
-   `switch <config>`: Switch to a specific config collection.
-   `doctor`: Show current configs and symlink status.
-   `purge`: Remove symlinks.
-   `repair`: Attempt to fix broken symlinks.
