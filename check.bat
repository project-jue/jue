@echo off
REM Project Jue Code Validation Script (Windows)
REM This script runs the complete validation pipeline as specified in the SWE guidelines

REM Color codes for output
for /F "tokens=1,2 delims=#" %%a in ('"prompt #$H#$E# & echo on & for %%b in (1) do rem"') do (
  set "DEL=%%a"
)

call :ColorText 0e "=== Project Jue Validation Pipeline ==="
call :ColorText 0e "Running complete code validation..."
echo.

REM 1. Formatting Check
call :ColorText 0e "[1/6] Checking code formatting..."
cargo fmt --check
if %ERRORLEVEL% equ 0 (
    call :ColorText 0a "✓ Formatting check passed"
) else (
    call :ColorText 0c "✗ Formatting issues found"
    call :ColorText 0e "Run 'cargo fmt' to fix formatting issues"
    exit /b 1
)

REM 2. Linting
call :ColorText 0e "[2/6] Running clippy linting..."
cargo clippy --all-targets --all-features -- -D warnings
if %ERRORLEVEL% equ 0 (
    call :ColorText 0a "✓ Linting passed"
) else (
    call :ColorText 0c "✗ Linting issues found"
    exit /b 1
)

REM 3. Unit Tests
call :ColorText 0e "[3/6] Running unit tests..."
cargo test --lib
if %ERRORLEVEL% equ 0 (
    call :ColorText 0a "✓ Unit tests passed"
) else (
    call :ColorText 0c "✗ Unit tests failed"
    exit /b 1
)

REM 4. Integration Tests
call :ColorText 0e "[4/6] Running integration tests..."
cargo test --test '*'
if %ERRORLEVEL% equ 0 (
    call :ColorText 0a "✓ Integration tests passed"
) else (
    call :ColorText 0c "✗ Integration tests failed"
    exit /b 1
)

REM 5. Documentation Tests
call :ColorText 0e "[5/6] Running documentation tests..."
cargo test --doc
if %ERRORLEVEL% equ 0 (
    call :ColorText 0a "✓ Documentation tests passed"
) else (
    call :ColorText 0c "✗ Documentation tests failed"
    exit /b 1
)

REM 6. Coverage (if tarpaulin is available)
call :ColorText 0e "[6/6] Checking test coverage..."
where cargo-tarpaulin >nul 2>&1
if %ERRORLEVEL% equ 0 (
    call :ColorText 0e "Running coverage analysis..."
    cargo tarpaulin --out Xml --output-dir target\coverage
    if %ERRORLEVEL% equ 0 (
        call :ColorText 0a "✓ Coverage report generated"
        call :ColorText 0e "Coverage report available in target\coverage\"
    ) else (
        call :ColorText 0c "✗ Coverage analysis failed"
        exit /b 1
    )
) else (
    call :ColorText 0e "⚠ cargo-tarpaulin not installed, skipping coverage check"
    call :ColorText 0e "Install with: cargo install cargo-tarpaulin"
)

echo.
call :ColorText 0a "=== All validation checks passed! ==="
call :ColorText 0a "✓ Formatting: OK"
call :ColorText 0a "✓ Linting: OK"
call :ColorText 0a "✓ Unit Tests: OK"
call :ColorText 0a "✓ Integration Tests: OK"
call :ColorText 0a "✓ Documentation Tests: OK"
call :ColorText 0a "✓ Coverage: OK (if available)"
echo.

REM Additional validation for documentation structure
call :ColorText 0e "Validating documentation structure..."

REM Check required documentation directories
set REQUIRED_DIRS=docs\cheatsheets docs\adr docs\design docs\subsystems docs\prompts

for %%d in (%REQUIRED_DIRS%) do (
    if exist "%%d" (
        call :ColorText 0a "✓ %%d exists"
    ) else (
        call :ColorText 0c "✗ %%d missing"
        exit /b 1
    )
)

REM Check required cheatsheet files
set REQUIRED_CHEATSHEETS=docs\cheatsheets\environment.md docs\cheatsheets\testing.md docs\cheatsheets\filesystem.md docs\cheatsheets\llm_integration.md

for %%f in (%REQUIRED_CHEATSHEETS%) do (
    if exist "%%f" (
        call :ColorText 0a "✓ %%f exists"
    ) else (
        call :ColorText 0c "✗ %%f missing"
        exit /b 1
    )
)

call :ColorText 0a "✓ Documentation structure validation passed"
echo.
call :ColorText 0a "=== Complete validation successful! ==="
call :ColorText 0a "Project is ready for deployment or further development."
goto :eof

:ColorText
echo off
<nul set /p ".=%DEL%" > "%~2"
findstr /v /a:%1 /R "^$" "%~2" nul
del "%~2" > nul 2>&1
goto :eof