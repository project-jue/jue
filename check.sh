#!/bin/bash

# Project Jue Code Validation Script
# This script runs the complete validation pipeline as specified in the SWE guidelines

# Exit on any error
set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Project Jue Validation Pipeline ===${NC}"
echo -e "${YELLOW}Running complete code validation...${NC}"

# 1. Formatting Check
echo -e "${YELLOW}[1/6] Checking code formatting...${NC}"
if cargo fmt --check; then
    echo -e "${GREEN}✓ Formatting check passed${NC}"
else
    echo -e "${RED}✗ Formatting issues found${NC}"
    echo -e "${YELLOW}Run 'cargo fmt' to fix formatting issues${NC}"
    exit 1
fi

# 2. Linting
echo -e "${YELLOW}[2/6] Running clippy linting...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✓ Linting passed${NC}"
else
    echo -e "${RED}✗ Linting issues found${NC}"
    exit 1
fi

# 3. Unit Tests
echo -e "${YELLOW}[3/6] Running unit tests...${NC}"
if cargo test --lib; then
    echo -e "${GREEN}✓ Unit tests passed${NC}"
else
    echo -e "${RED}✗ Unit tests failed${NC}"
    exit 1
fi

# 4. Integration Tests
echo -e "${YELLOW}[4/6] Running integration tests...${NC}"
if cargo test --test '*'; then
    echo -e "${GREEN}✓ Integration tests passed${NC}"
else
    echo -e "${RED}✗ Integration tests failed${NC}"
    exit 1
fi

# 5. Documentation Tests
echo -e "${YELLOW}[5/6] Running documentation tests...${NC}"
if cargo test --doc; then
    echo -e "${GREEN}✓ Documentation tests passed${NC}"
else
    echo -e "${RED}✗ Documentation tests failed${NC}"
    exit 1
fi

# 6. Coverage (if tarpaulin is available)
echo -e "${YELLOW}[6/6] Checking test coverage...${NC}"
if command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${YELLOW}Running coverage analysis...${NC}"
    if cargo tarpaulin --out Xml --output-dir target/coverage; then
        echo -e "${GREEN}✓ Coverage report generated${NC}"
        echo -e "${YELLOW}Coverage report available in target/coverage/${NC}"
    else
        echo -e "${RED}✗ Coverage analysis failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ cargo-tarpaulin not installed, skipping coverage check${NC}"
    echo -e "${YELLOW}Install with: cargo install cargo-tarpaulin${NC}"
fi

echo -e "${GREEN}"
echo -e "=== All validation checks passed! ==="
echo -e "✓ Formatting: OK"
echo -e "✓ Linting: OK"
echo -e "✓ Unit Tests: OK"
echo -e "✓ Integration Tests: OK"
echo -e "✓ Documentation Tests: OK"
echo -e "✓ Coverage: OK (if available)"
echo -e "${NC}"

# Additional validation for documentation structure
echo -e "${YELLOW}Validating documentation structure...${NC}"

# Check required documentation directories
REQUIRED_DIRS=(
    "docs/cheatsheets"
    "docs/adr"
    "docs/design"
    "docs/subsystems"
    "docs/prompts"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo -e "${GREEN}✓ $dir exists${NC}"
    else
        echo -e "${RED}✗ $dir missing${NC}"
        exit 1
    fi
done

# Check required cheatsheet files
REQUIRED_CHEATSHEETS=(
    "docs/cheatsheets/environment.md"
    "docs/cheatsheets/testing.md"
    "docs/cheatsheets/filesystem.md"
    "docs/cheatsheets/llm_integration.md"
)

for file in "${REQUIRED_CHEATSHEETS[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ $file exists${NC}"
    else
        echo -e "${RED}✗ $file missing${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✓ Documentation structure validation passed${NC}"

echo -e "${GREEN}"
echo -e "=== Complete validation successful! ==="
echo -e "Project is ready for deployment or further development."
echo -e "${NC}"