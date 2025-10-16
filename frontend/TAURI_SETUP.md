# Tauri Desktop App Setup

## Quick Start

To run the Lightweight app as a native desktop application:

### 1. Install Tauri CLI (one-time)

```bash
cargo install tauri-cli
```

### 2. Initialize Tauri in this project

```bash
cd frontend
cargo tauri init
```

When prompted:
- **App name**: `Lightweight`
- **Window title**: `Lightweight Fitness Tracker`
- **Web assets location**: `../build` (SvelteKit build output)
- **Dev server URL**: `http://localhost:5173`
- **Frontend dev command**: `npm run dev`
- **Frontend build command**: `npm run build`

### 3. Run in development

```bash
# Terminal 1: Run the gRPC backend
cd ../web
cargo run

# Terminal 2: Run the Tauri desktop app
cd ../frontend
cargo tauri dev
```

### 4. Build for production

```bash
npm run build
cargo tauri build
```

The built app will be in `src-tauri/target/release/`.

## How It Works

- **Tauri** wraps your SvelteKit app in a native window
- The same TypeScript gRPC clients work unchanged
- Small bundle size (~5MB) compared to Electron (~100MB+)
- Native system APIs available via Rust if needed
- Cross-platform: builds for macOS, Windows, Linux

## Configuration

After initialization, you can customize:
- `src-tauri/tauri.conf.json` - App settings, window size, permissions
- `src-tauri/src/main.rs` - Rust backend for native features
- `src-tauri/Cargo.toml` - Rust dependencies

## Mobile Support (Future)

For iOS/Android, consider using Capacitor instead of Tauri:

```bash
npm install @capacitor/core @capacitor/cli
npx cap init
npx cap add ios
npx cap add android
```

The same gRPC clients will work with Capacitor too!
