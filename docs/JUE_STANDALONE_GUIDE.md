# Jue Standalone Executable Guide

## 🎯 Quick Start

You can now run Jue files directly using the standalone `jue` executable!

### Installation

```bash
# Build and install the jue executable
cargo install --path .

# This installs the executable as `jue`
```

### Basic Usage

```bash
# Run a Jue file directly
jue myfile.jue

# With debug output
jue myfile.jue --debug

# Different trust tier
jue myfile.jue --tier formal
```

### Verified Working Examples

✅ **Basic Execution**: `jue jue_examples/hello_world.jue`
✅ **Debug Mode**: `jue jue_examples/hello_world.jue --debug`
✅ **AST Printing**: `jue jue_examples/hello_world.jue --print-ast`
✅ **Help System**: `jue --help`
✅ **REPL Mode**: `jue --interactive`

## 🚀 Standalone Executable Features

The standalone `jue` executable provides all the same features as the development version:

### Command Line Interface

```bash
jue [OPTIONS] [file]
```

### All Options Available

```bash
# Show help
jue --help

# Interactive REPL mode
jue --interactive

# Print AST
jue myfile.jue --print-ast

# Print bytecode
jue myfile.jue --print-bytecode

# Custom resource limits
jue myfile.jue --steps 5000 --memory 2097152
```

## 📋 Example Usage

### 1. Simple Execution

```bash
jue jue_examples/hello_world.jue
```

Output:
```
Executing jue_examples/hello_world.jue with empirical tier

=== Execution Results ===
Output: 42

Resource Usage:
  Steps: 1
  Memory: 0 bytes
  Time: 1.8066ms
```

### 2. Debug Mode

```bash
jue jue_examples/hello_world.jue --debug
```

### 3. Different Trust Tiers

```bash
# Formal tier (highest verification)
jue jue_examples/hello_world.jue --tier formal

# Experimental tier (sandboxed)
jue jue_examples/hello_world.jue --tier experimental
```

### 4. Resource Limits

```bash
# Increase step limit
jue jue_examples/hello_world.jue --steps 100000

# Increase memory limit
jue jue_examples/hello_world.jue --memory 4194304  # 4MB
```

## 🔧 Installation Methods

### Method 1: Cargo Install (Recommended)

```bash
# Install from current directory
cargo install --path .

# This creates: ~/.cargo/bin/jue
```

### Method 2: Build Release Binary

```bash
# Build release version
cargo build --release

# The binary is at: target/release/jue
```

### Method 3: Copy to System Path

```bash
# Copy to a directory in your PATH
cp target/release/jue /usr/local/bin/jue

# Make executable
chmod +x /usr/local/bin/jue
```

## 🐛 Troubleshooting

### "Command not found"

If you get `jue: command not found`, ensure:

1. **Cargo bin directory is in PATH**:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

2. **Executable is installed**:
   ```bash
   ls ~/.cargo/bin/jue
   ```

3. **Symlink exists** (if using `jue` name):
   ```bash
   ln -s ~/.cargo/bin/jue ~/.cargo/bin/jue
   ```

### Permission Issues

```bash
chmod +x ~/.cargo/bin/jue
```

## 🎓 Advanced Usage

### Batch Processing

```bash
#!/bin/bash
for file in *.jue; do
    echo "Processing $file..."
    jue "$file" --tier empirical
    echo "------------------"
done
```

### Alias for Convenience

Add to your `.bashrc` or `.zshrc`:

```bash
alias jue='jue'
```

## 📈 Performance Tips

1. **Use release builds** for production:
   ```bash
   cargo build --release
   ```

2. **Set appropriate resource limits** based on your program complexity

3. **Use empirical tier** for most development work

4. **Use formal/verified tiers** only when you need proof generation

## 🔍 Understanding the Executable

The standalone `jue` executable includes:

- **Complete Jue compiler** with all trust tier support
- **Physics World VM** for execution
- **Capability system** with runtime checks
- **Resource management** with configurable limits
- **Error handling** with clear messages
- **Debug tools** for development

## 📋 Summary

You now have a fully functional standalone Jue executable that can be used like any other command-line tool:

✅ **Direct file execution**: `jue myfile.jue`
✅ **All command-line options**: `--debug`, `--tier`, `--steps`, etc.
✅ **Interactive REPL**: `jue --interactive`
✅ **Installation options**: Cargo install, manual copy, symlinks
✅ **Cross-platform**: Works on Windows, macOS, Linux

Start using Jue today with the simple command:

```bash
jue your_program.jue