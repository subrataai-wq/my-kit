# Prelude Desktop App

A modern cross-platform desktop application built with [Dioxus 0.7](https://dioxuslabs.com/) and Rust, featuring a modular, tiered utility architecture, persistent local storage, and an accessible UI component kit styled with CSS variables.

---

## Project File Structure

```
my_desktop_app/
├── assets/                          # Static assets and global styling
│   ├── favicon.ico                  # Application icon
│   ├── header.svg                   # Brand illustration
│   ├── main.css                     # Semantic CSS variables & component styles
│   └── tailwind.css                 # Optional utility CSS
├── src/
│   ├── lib.rs                       # Library entrypoint exposing `utils`
│   ├── main.rs                      # Dioxus desktop application entrypoint & root UI
│   └── utils/                       # Tiered utility architecture
│       ├── mod.rs                   # Main utils facade & unified prelude
│       ├── base/                    # [Zero outside crates] Pure primitives & algorithms
│       │   ├── mod.rs
│       │   ├── collections.rs       # chunk, dedupe, partition, frequency_map, Progress
│       │   ├── id.rs                # short_id, unique_id, random_u64, random_range
│       │   ├── math.rs              # clamp, lerp, percent, round_to, stats, gcd, lcm
│       │   ├── result.rs            # AppError, AppResult, Context trait, ErrorKind
│       │   ├── text.rs              # truncate, slugify, capitalize, pluralize, mask
│       │   ├── time.rs              # human_ago, format_duration, unix_to_iso8601
│       │   └── validate.rs          # is_email, is_http_url, is_username, in_range
│       ├── platform/                # System & OS abstractions
│       │   ├── mod.rs
│       │   ├── env.rs               # Environment variables & .env parsing
│       │   ├── fs.rs                # Atomic writes, directory traversal, file size
│       │   ├── paths.rs             # app_data_dir, app_config_dir, sanitize_filename
│       │   └── storage.rs           # KeyValueStore trait, FileStorage, MemoryStorage
│       ├── data/                    # Parsing, querying, and pagination
│       │   ├── mod.rs
│       │   ├── csv.rs               # RFC 4180 CSV parser & serializer
│       │   ├── json.rs              # JSON serialization & file persistence
│       │   ├── paginate.rs          # Pagination struct, slicing with div_ceil
│       │   ├── search.rs            # fuzzy_match, fuzzy_score, zero-copy filter_by
│       │   └── sort.rs              # natural_cmp ("item2" < "item10"), SortOrder
│       ├── checksum/                # Data integrity checksums
│       │   ├── mod.rs
│       │   ├── crc32.rs             # IEEE 802.3 32-bit CRC checksum
│       │   └── fnv.rs               # 32-bit and 64-bit FNV-1a checksums
│       ├── net/                     # Network resilience & caching
│       │   ├── mod.rs
│       │   ├── cache.rs             # TtlCache (explicit 'now' parameter, wasm-safe)
│       │   ├── http.rs              # HttpMethod, QueryParams builder
│       │   └── retry.rs             # retry_async with exponential backoff
│       ├── state/                   # Dioxus 0.7 reactive hooks
│       │   ├── mod.rs
│       │   ├── debounce.rs          # Debouncer rate-limiting helper
│       │   ├── notice.rs            # use_notices() toast & status alert manager
│       │   ├── persist.rs           # use_persistent() state synced to FileStorage
│       │   └── use_now.rs           # use_now() live reactive clock signal hook
│       └── ui/                      # Dioxus 0.7 components with CSS variables
│           ├── mod.rs
│           ├── badge.rs             # Status pill badge component
│           ├── banner.rs            # Alert banners with dismiss buttons
│           ├── button.rs            # Button with variants (Primary, Danger, Ghost)
│           ├── empty_state.rs       # Empty state graphic & placeholder
│           ├── modal.rs             # Accessible modal (Escape key, a11y, backdrop)
│           ├── progress_bar.rs      # ProgressBar driven by Progress metrics
│           └── theme.rs             # use_theme() context hook (Light / Dark)
├── Cargo.lock
├── Cargo.toml                       # Package manifest & granular feature flags
├── clippy.toml                      # Lint configuration
├── Dioxus.toml                      # Dioxus platform & app metadata
└── AGENTS.md                        # Agent & developer architectural guidelines
```

---

## Utility Architecture Overview

The utility toolkit follows strict layer boundaries:
- **`base/`**: Completely independent with **zero external dependencies**. Contains pure algorithms and data structures.
- **`platform/`**: Handles OS-level operations (files, directories, environment, key-value storage).
- **`data/`**: Data formatting (CSV, JSON), natural sorting, fuzzy searching, and pagination.
- **`checksum/`**: Fast non-cryptographic checksums (`crc32`, `fnv`). Passwords defer to `argon2`; secrets defer to `zeroize`.
- **`net/`**: Safe async retry and TTL caching (taking explicit `now` parameter to guarantee wasm safety).
- **`state/`**: Reactive state management hooks (`use_persistent`, `use_notices`, `use_now`, `debounce`).
- **`ui/`**: Accessible presentation components styled via central CSS variables in `assets/main.css`.

---

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (edition 2021)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started):
  ```bash
  cargo install dioxus-cli
  ```

### Running the App
Run with the default platform (web/desktop):
```bash
dx serve
```

Run directly on desktop:
```bash
dx serve --platform desktop
```

Or run via standard Cargo:
```bash
cargo run
```

---

## Testing & Quality Assurance

Run the test suite (52 unit tests across all 7 utility modules):
```bash
cargo test
```

Verify zero-dependency library compilation:
```bash
cargo check --no-default-features --lib
```

Run Clippy strict lints:
```bash
cargo clippy -- -D warnings
```
