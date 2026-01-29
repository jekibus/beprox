# BeProx - Development Plan

**Goal**: Build a desktop application like [LocalCan](https://www.localcan.com) using Tauri, enabling developers to expose local servers via `.local` domains and public URLs, with traffic inspection capabilities.

## Phase 1: MVP - Local Domain Proxy 🚀
- [x] **Setup Tauri Workspace**
    - Initialize Tauri with **Svelte**, TypeScript, and Vite using **Bun**.
    - Configure TailwindCSS (or similar) for UI.
- [x] **Local Proxy Engine (Rust)**
    - Implement a reverse proxy server (using `pingora`, `hyper`, or `axum`).
    - Bind to port 80 (requires admin privileges).
    - Forward traffic from `myapp.local` -> `localhost:5173`.
- [ ] **DNS / Host Resolution**
    - Modify `/etc/hosts` to map `myapp.local` -> `127.0.0.1`.

## Phase 2: Traffic Inspector 🔍
- [ ] **Request/Response Capture**
- [ ] **HTTPS / TLS Support**
    - Generate a local Root CA.
    - Dynamically generate certificates for user-defined domains.
    - Install/Trust Root CA in the system keychain (security command).

## Phase 3: Traffic Inspector 🔍
- [ ] **Request/Response Capture**
    - Implement middleware in the Rust proxy to capture HTTP metadata (Headers, Body, Status, Timing).
- [ ] **Real-time UI Updates**
    - Stream logs from Rust to Frontend via Tauri Events.
    - Create a "Network" tab in UI similar to Chrome DevTools.
- [ ] **Replay Mechanism**
    - Ability to resend a captured request from the backend.

## Phase 4: Public URLs (Tunneling) 🌍
- [ ] **Tunneling Client**
    - Implement a tunneling client to expose local ports to the internet.
    - *Architecture*: Need a relay server (VPS).
    - *Protocol*: SSH Remote Forwarding, WebSocket Tunnel, or HTTP/2 Tunnel.
    - *MVP*: Use an existing open-source tunnel solution (like `rathole` or similar) embedded or re-implemented.

## Phase 5: Advanced Features & Polish ✨
- [ ] **Privileged Helper Tool**
    - Separate binary to handle root-level tasks (binding port 80, editing hosts) to avoid repeated password prompts.
- [ ] **Persistent State**
    - Save configurations to disk (SQLite or JSON).
- [ ] **Tray Icon**
    - Quick access to start/stop services.

## Technical Stack
- **Frontend**: Svelte, TypeScript, TailwindCSS, Vite.
- **Backend (Tauri)**: Rust.
- **Proxy**: `hyper` / `tower` / `pingora`.
- **State Management**: React Query / Zustand.
