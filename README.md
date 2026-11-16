# my-kit

> Reusable Multi-Crate Engine & Accessible UI Framework for Rust Ventures

`my-kit` is the foundational technology engine powering the Startup Studio. Engineered in **Rust (edition 2024)** and built for **Dioxus 0.7**, it provides zero-UI headless primitives, panic-free multi-target storage, memory-efficient string handling, timestamp-sortable identifiers, and an accessible presentation component library styled via semantic CSS variables.

---

## Workspace Architecture

`my-kit` is organized as a Cargo workspace containing three distinct, decoupled crates:

```
my-kit/
├── .cargo/
│   └── config.toml                  # WASM configuration (getrandom wasm_js backend)
├── .github/
│   └── workflows/
│       └── ci.yml                   # CI pipeline (Ubuntu, Windows, and wasm32)
├── assets/                          # Design tokens and shared branding
│   ├── favicon.ico
│   ├── header.svg
│   ├── main.css                     # Semantic CSS design tokens & component classes
│   └── tailwind.css
├── crates/
│   ├── my-kit-core/                 # [Tier 1] Headless engine (Zero Dioxus dependency)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs               # Exports & unified prelude
│   │       ├── auth/                # TokenSession, Claims, secrecy, AuthError
│   │       ├── base/                # id (ulid), math, result (thiserror), text, time, validate
│   │       ├── checksum/            # crc32, fnv1a
│   │       ├── data/                # csv, json, paginate, fuzzy search, natural sorting
│   │       ├── forms/               # Field<T>, CompactStr validation rules
│   │       ├── media/               # Aspect ratio math, cover/contain dimension calculations
│   │       ├── net/                 # TTL cache, query param builder, async retry
│   │       └── platform/            # env, fs, paths, KeyValueStore (Memory, File, LocalStorage)
│   ├── my-kit-ui/                   # [Tier 2] Dioxus 0.7 Presentation & Reactive Hooks
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/               # use_theme, use_notices, use_persistent, use_now, debounce
│   │       └── ui/                  # Button, FormField, TextInput, Modal, Badge, Banner, etc.
│   └── my-kit-template/             # [Tier 3] Showcase application & validation harness
│       ├── Cargo.toml
│       ├── Dioxus.toml
│       └── src/
│           └── main.rs              # Interactive demo of forms, storage, and theme switching
├── Cargo.lock
├── Cargo.toml                       # Workspace manifest & shared dependencies
├── clippy.toml                      # Strict zero-warning lint configuration
└── Dioxus.toml
```

---

## Core Capabilities & Integrated Crates

### 1. `my-kit-core` (Headless Domain Layer)
- **Universal Target Compatibility:** Compiles cleanly to both 64-bit native OS targets and `wasm32-unknown-unknown`.
- **Structured Domain Errors (`thiserror`):** Typed error enums (`StorageError`, `AuthError`, `AppError`) with automatic source chaining and backward-compatible `AppResult`.
- **Zero-Allocation Small Strings (`compact_str`):** `Field<T>.error` and `Claims.roles` use `CompactString` (aliased as `CompactStr`), storing strings up to 24 bytes directly on the stack without heap allocation.
- **Monotonic 128-bit Identifiers (`ulid`):** `new_ulid()` and `new_monotonic_ulid_string()` provide 26-character Crockford Base32 IDs that encode UNIX millisecond timestamps for instant chronological sorting.
- **Cryptographic Memory Safety (`secrecy`):** Wraps authentication tokens in `SecretString` to automatically redact secrets in `Debug`/`tracing` logs and wipe memory upon drop.
- **Panic-Free Persistence (`storage`):** Unified `KeyValueStore` trait implemented by:
  - `MemoryStorage`: Thread-safe `Arc<RwLock<HashMap>>` for testing and in-memory caches.
  - `FileStorage`: Atomic disk persistence with tempfile swapping (Native only).
  - `LocalStorage`: Browser persistence that gracefully handles Web Worker and SSR environments where `window` is unavailable.

### 2. `my-kit-ui` (Presentation Layer)
- **Dioxus 0.7 Native:** Leverages modern signal reactivity (`use_signal`, `Memo`), tree-shaken RSX, and component prop definitions.
- **Reactive State Hooks:**
  - `use_theme()`: Light/Dark theme switching persisted across sessions.
  - `use_notices()`: Toast and global alert dispatching (`success`, `error`, `info`).
  - `use_persistent()`: State synchronized automatically with underlying `KeyValueStore`.
  - `use_now()`: Reactive live timestamp tick for real-time humanized relative times.
  - `use_debounce()`: Event rate-limiting for search inputs and rapid triggers.
- **Semantic Component Kit:** Pre-styled accessible components (`Button`, `FormField`, `TextInput`, `Modal`, `Badge`, `Banner`, `ProgressBar`, `EmptyState`) driven by CSS variables defined in `assets/main.css`.

---

## How Client Ventures Consume `my-kit`

### 1. Remote Production Release (`Cargo.toml`)
Client applications import the engine via GitHub tag:

```toml
[dependencies]
my-kit-core = { git = "https://github.com/subrataai-wq/my-kit.git", tag = "v0.1.0" }
my-kit-ui = { git = "https://github.com/subrataai-wq/my-kit.git", tag = "v0.1.0" }
```

### 2. Fast Local Development Patch (`.cargo/config.toml`)
For instantaneous local builds, client repositories contain a git-ignored `.cargo/config.toml` that redirects GitHub dependencies to local disk:

```toml
[patch."https://github.com/subrataai-wq/my-kit.git"]
my-kit-core = { path = "../../../my-kit/crates/my-kit-core" }
my-kit-ui = { path = "../../../my-kit/crates/my-kit-ui" }
```

---

## Developer Workflow & Quality Assurance

### Prerequisites
- **Rust Toolchain:** `1.85+` (Rust edition 2024 supported)
- **WebAssembly Target:**
  ```powershell
  rustup target add wasm32-unknown-unknown
  ```
- **Dioxus CLI (v0.7.1+):**
  ```powershell
  cargo install dioxus-cli --version 0.7.1
  ```

### Running the Showcase Application

Run on Web (default):
```powershell
cd crates/my-kit-template
dx serve --platform web
```

Run on Desktop:
```powershell
cd crates/my-kit-template
dx serve --platform desktop
```

### Verification & Test Suite

Run the full workspace unit test suite (**65 tests passing**):
```powershell
cargo test --workspace
```

Verify `my-kit-core` WebAssembly compilation:
```powershell
cargo check -p my-kit-core --target wasm32-unknown-unknown
```

Run strict Clippy lint checks (**0 warnings enforced**):
```powershell
cargo clippy --workspace --all-targets -- -D warnings
```

---

## License

Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
