# Filesystem Cheat Sheet

## Project Structure

### Directory Layout
```
project_root/
├── core_world/          # Formal λ-calculus kernel
├── physics_world/       # Deterministic VM
├── dan_world/           # Cognitive layer
├── docs/                # Documentation
├── spec/                # Formal specifications
├── tests/               # Integration tests
├── benches/             # Performance tests
└── src/                 # Main source code
```

### Key File Locations
- `Cargo.toml`: Project manifest
- `README.md`: Project overview
- `CHANGELOG.md`: Version history
- `.gitignore`: Git ignore patterns

## File Operations

### Common File Commands

**Windows (default):**
```cmd
:: List files
dir

:: List files recursively
dir /s

:: Create directory
mkdir directory_name

:: Remove directory (empty)
rmdir directory_name

:: Remove directory with contents
rmdir /s /q directory_name

:: Copy file
copy source.txt destination.txt

:: Move file
move source.txt destination.txt

:: Delete file
del file.txt

:: Find files
dir /s *pattern*
```

**Linux:**
```bash
# List files
ls -la

# List files recursively
ls -laR

# Create directory
mkdir directory_name

# Remove directory (empty)
rmdir directory_name

# Remove directory with contents
rm -rf directory_name

# Copy file
cp source.txt destination.txt

# Move file
mv source.txt destination.txt

# Delete file
rm file.txt

# Find files
find . -name "*pattern*"
```

## File Navigation

### Navigation Commands

**Windows (default):**
```cmd
:: Change directory
cd path\to\directory

:: Go up one level
cd ..

:: Go to root
cd \

:: Show current directory
cd

:: Create symbolic link (requires admin)
mklink /D link_name target_path
```

**Linux:**
```bash
# Change directory
cd path/to/directory

# Go up one level
cd ..

# Go to root
cd /

# Show current directory
pwd

# Create symbolic link
ln -s target_path link_name
```

## File Permissions

### Permission Management

**Windows (default):**
```cmd
:: View file permissions
icacls file.txt

:: Grant read permission
icacls file.txt /grant User:(R)

:: Grant full control
icacls file.txt /grant User:(F)

:: Remove permissions
icacls file.txt /remove User
```

**Linux:**
```bash
# View file permissions
ls -la file.txt

# Change permissions (chmod)
chmod 644 file.txt      # rw-r--r--
chmod 755 file.txt      # rwxr-xr-x
chmod +x file.txt       # Add execute permission

# Change ownership (chown)
sudo chown user:group file.txt

# Change group (chgrp)
sudo chgrp group file.txt
```

## File Content Operations

### Content Management

**Windows (default):**
```cmd
:: View file content
type file.txt

:: View file with line numbers
find /n /v "" file.txt

:: Search in file
find "pattern" file.txt

:: Count lines
find /c /v "" file.txt

:: File statistics
for %F in (file.txt) do @echo %~zF bytes
```

**Linux:**
```bash
# View file content
cat file.txt

# View file with line numbers
cat -n file.txt

# Search in file
grep "pattern" file.txt

# Count lines
wc -l file.txt

# File statistics
wc file.txt
stat file.txt
```

## Project-Specific Operations

### Rust Project Files
```bash
# Create new Rust project
cargo new project_name

# Add dependency
cargo add dependency_name

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### Documentation Files
```bash
# Generate documentation
cargo doc --open

# Build documentation
cargo doc --no-deps

# Documentation location
target/doc/
```

## Cross-Platform File Handling

### Path Handling in Code
```rust
// Use std::path::Path for cross-platform path handling
use std::path::Path;

let path = Path::new("path/to/file");
let windows_path = Path::new(r"C:\path\to\file");

// Convert to string
let path_str = path.to_string_lossy();
```

### File I/O Operations
```rust
use std::fs;
use std::io::{self, Read, Write};

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn create_dir(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}
```

## File Best Practices

### Organization
- Keep related files together in directories
- Use consistent naming conventions
- Document file purposes in READMEs
- Avoid deep nesting (max 3-4 levels)

### Naming
- Use lowercase with underscores (`snake_case`)
- Be descriptive but concise
- Avoid special characters and spaces
- Use consistent extensions

### Version Control
- Commit small, focused changes
- Write descriptive commit messages
- Use `.gitignore` appropriately
- Avoid committing build artifacts