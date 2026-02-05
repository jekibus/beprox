# Changelog

All notable changes to BeProx will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.7] - 2026-02-05

### Added
- **Cross-Platform Support**: Added support for Windows hosts file modification (`C:\Windows\System32\drivers\etc\hosts`).
- **Linux Support**: Validated support for Linux `/etc/hosts` modification.

### Changed
- **Host Management**: Updated hosts file logic to be platform-aware using Rust `cfg` attributes.
- **Error Messages**: Improved permission error messages to be specific to the operating system (Administrator vs sudo).

## [0.0.6] - 2026-02-03

### Changed
- **Branding**: Updated application icons and favicons across Desktop App, Web Interface, and Documentation.
- **System Tray**: Fixed system tray icon not loading by explicitly embedding the custom icon resource in Rust.
- **Dependencies**: Added `image-png` feature to `tauri` crate to support dynamic icon loading.

## [0.0.5] - 2026-01-30

### Added
- **Documentation**: Initialized VitePress documentation in `docs/` directory.
- **CI/CD**: Added GitHub Actions workflow (`deploy-docs.yml`) for automatic deployment to GitHub Pages.
- **Scripts**: Added `docs:dev`, `docs:build`, and `docs:preview` scripts to `package.json`.

### Changed
- **Readme**: Updated `README.md` to serve as a landing page linking to full documentation.
- **Plan**: Migrated development plan from `PLAN.md` to `docs/roadmap.md`.

## [0.0.4] - 2026-01-29

### Added
- **Settings**: Added global settings management for the application.
- **Default TLD**: Added ability to configure default Top-Level Domain (e.g., `.local`) in settings.
- **UI**: Implemented modern UI using TailwindCSS with a clean dashboard layout.

### Improved
- **UX**: Added loading states and better error handling for site creation.
- **Navigation**: Added Settings modal and improved main navigation.

## [0.0.3] - 2026-01-28

### Added
- **Host Management**: Implemented automatic `/etc/hosts` file modification.
- **Safe Editing**: Added `## BeProx - Start ##` markers to safely manage custom entries without affecting other system configurations.
- **Commands**: Added `add_host_entry` and `remove_host_entry` Tauri commands.

### Secured
- **Validation**: Added checks to ensure host file integrity before writing.

## [0.0.2] - 2026-01-27

### Added
- **Proxy Engine**: Implemented core reverse proxy logic using `axum` and `reqwest`.
- **Port 80**: Added capability to bind to port 80 (requires administrative privileges).
- **Traffic Forwarding**: Implemented request forwarding from custom domains to local ports (e.g., `myapp.local` -> `localhost:5173`).

### Fixed
- **Headers**: Implemented filtering of hop-by-hop headers to ensure correct proxy behavior.

## [0.0.1] - 2026-01-27

### Added
- **Project Birth**: Initial setup of Tauri v2 workspace.
- **Stack**: configured Svelte 5, TypeScript, and Bun.
- **Backend**: Initialized Rust backend structure with `src-tauri`.


## Guide

## Version Format

This project follows semantic versioning:

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards compatible manner
- **PATCH**: Backwards compatible bug fixes

## Category Format

- Added: New features or components
- Changed: Modifications to existing features
- Fixed: Bug fixes
- Improved: Enhancements in speed or efficiency
- Secured: Enhancements to protect user data
