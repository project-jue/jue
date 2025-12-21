#!/bin/bash

# Build and copy the jue executable to the examples folder

# Build the release version
echo "Building jue executable..."
cargo build --release

# Check if build succeeded
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Create jue_examples directory if it doesn't exist
mkdir -p jue_examples

# Copy the executable to jue_examples folder
echo "Copying jue executable to jue_examples folder..."
cp target/release/jue jue_examples/jue

# Check if copy succeeded
if [ $? -ne 0 ]; then
    echo "Copy failed!"
    exit 1
fi

echo "✅ Successfully built and copied jue executable to jue_examples/jue"
echo "You can now run: ./jue_examples/jue your_file.jue"