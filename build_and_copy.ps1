# Build and copy the jue executable to the examples folder

# Build the release version
Write-Host "Building jue executable..."
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}

# Create jue_examples directory if it doesn't exist
if (!(Test-Path -Path "jue_examples")) {
    New-Item -ItemType Directory -Path "jue_examples" | Out-Null
}

# Copy the executable to jue_examples folder
Write-Host "Copying jue executable to jue_examples folder..."
Copy-Item -Path "target\release\jue.exe" -Destination "jue_examples\jue.exe" -Force

if ($LASTEXITCODE -ne 0) {
    Write-Error "Copy failed!"
    exit 1
}

Write-Host "✅ Successfully built and copied jue executable to jue_examples\jue.exe"
Write-Host "You can now run: .\jue_examples\jue.exe your_file.jue"