# BeProx

BeProx is a powerful desktop application for developers, designed to simplify local development by exposing local servers via custom `.local` domains and public URLs. It also aims to provide advanced traffic inspection capabilities.

## 🚀 Features

### Current Features (MVP)
- **Local Domain Proxy**: Map custom domains (e.g., `myapp.local`) to your local running services (e.g., `localhost:5173`).
- **Automatic Host Resolution**: Automatically manages `/etc/hosts` entries for your custom domains.
- **Port 80 Binding**: Runs on port 80 to allow accessing domains without specifying ports in the browser.
- **System Tray Integration**: Quick access to open the app or quit.

### Roadmap
- **Traffic Inspector**: Capture and inspect HTTP/HTTPS request/response metadata (Headers, Body, Timing).
- **HTTPS / TLS Support**: Dynamic certificate generation for local domains.
- **Public Tunneling**: Expose local ports to the internet via public URLs.
- **Replay Mechanism**: Replay captured requests for debugging.

## 🛠️ Tech Stack

- **Frontend**: [Svelte 5](https://svelte.dev/), [TypeScript](https://www.typescriptlang.org/), [TailwindCSS](https://tailwindcss.com/), [Vite](https://vitejs.dev/)
- **Backend**: [Tauri v2](https://tauri.app/) (Rust)
- **Runtime**: [Bun](https://bun.sh/)

## 📋 Prerequisites

Before running the project, ensure you have the following installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Bun**: [Install Bun](https://bun.sh/)
- **Tauri Prerequisites**: Follow the [Tauri System Configuration](https://tauri.app/v1/guides/getting-started/prerequisites) guide for your OS.

## 💻 Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/yourusername/beprox.git
cd beprox
```

### 2. Install dependencies
```bash
bun install
```

### 3. Run in Development Mode
Since BeProx needs to bind to port 80 and modify `/etc/hosts`, it requires administrative privileges during development.

```bash
sudo bun tauri dev
```
*Note: On macOS/Linux, `sudo` is required. On Windows, run your terminal as Administrator.*

### 4. Build for Production
To build the application for your operating system:

```bash
bun tauri build
```

The output binary/bundle will be located in:
- **macOS**: `src-tauri/target/release/bundle/macos/BeProx.app`
- **Windows**: `src-tauri/target/release/bundle/msi/` or `nsis/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## 📝 License

MIT
