# Environment Cheat Sheet

## System Information

### Operating Systems
- **Windows 10**: Default development environment
- **Linux**: Supported for deployment and testing

### Default Shells
- **Windows**: `C:\Windows\system32\cmd.exe`
- **Linux**: `/bin/bash`

## Development Tools

### Rust Setup
```bash
# Windows (default)
rustup default stable
rustup update

# Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Cargo Commands
```bash
# Build project (Windows default)
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test --test test_name

# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run clippy
cargo clippy

# Generate documentation
cargo doc --open
```

## Environment Variables

### Common Variables
- `RUST_LOG`: Controls logging level (`debug`, `info`, `warn`, `error`)
- `RUST_BACKTRACE`: Set to `1` for detailed backtraces
- `CARGO_TERM_COLOR`: Set to `always` for colored output

### Setting Variables

**Windows (default):**
```cmd
set RUST_LOG=debug
set RUST_BACKTRACE=1
```

**Linux:**
```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
```

## Project Structure

### Key Directories
- `core_world/`: Formal λ-calculus kernel
- `physics_world/`: Deterministic VM implementation
- `dan_world/`: Cognitive layer modules
- `docs/`: Documentation and guidelines
- `spec/`: Formal specifications

### Build Artifacts
- `target/debug/`: Debug builds
- `target/release/`: Release builds
- `target/doc/`: Generated documentation

## Cross-Platform Notes

### Path Handling
- Use forward slashes `/` in Rust code for cross-platform compatibility
- Windows paths in documentation should use backslashes `\`

### Command Equivalents

| Action                | Windows (default) | Linux     |
| --------------------- | ----------------- | --------- |
| List files            | `dir`             | `ls -la`  |
| Change directory      | `cd path`         | `cd path` |
| Current directory     | `cd`              | `pwd`     |
| Environment variables | `set`             | `env`     |
| Clear screen          | `cls`             | `clear`   |