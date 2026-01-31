# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-31

### Added
- **CLI Commands:**
    - `init`: Initialize Dotsy configuration.
    - `list`: List available configuration profiles.
    - `switch <config>`: Apply a specific configuration profile.
    - `doctor`: Validate symlink status and configuration health.
    - `purge`: Remove all active symlinks managed by Dotsy.
    - `repair`: Detect and fix broken or missing symlinks.
- **Features:**
    - `auto_detect_profile` setting: Automatically select a profile based on the system's hostname when no profile is specified in `dotsy switch`.
    - Dry-run mode (`--dry-run`) to preview filesystem changes.
    - Custom config path support (`--config`).
    - Configurable icon styles (Text, Emoji, NerdFont).
    - Pre-switch and Post-switch hooks for script execution.
    - Global and profile-specific symlink management.
- **Infrastructure:**
    - JSON Schema for `dotsy.toml` to provide LSP completions via Taplo.
    - Project Roadmap (`ROADMAP.md`).
    - GitLab CI configuration for automated multi-platform builds (x86_64, aarch64, musl) and releases.
    - `mise` tasks for containerized development and CI testing.

### Changed
- Refactored `on_conflict` configuration setting to use a typed enum instead of a string.
- Moved `ConflictAction` enum to a dedicated module in `src/config/conflict.rs`.
