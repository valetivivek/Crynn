#!/bin/bash

# Crynn Browser Optimized Build Script
# This script builds the browser with all optimizations for minimal size and memory usage

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "Building Crynn Browser with optimizations..."
echo "============================================"

# Set optimization environment variables
export RUSTFLAGS="-C target-cpu=native -C opt-level=z -C panic=abort -C strip=symbols"

# Build with optimizations
echo "Building release binary..."
cargo build --release --package crynn-shell

# Get binary info
BINARY="$PROJECT_ROOT/target/release/crynn-shell"
if [ -f "$BINARY" ]; then
    SIZE=$(du -h "$BINARY" | cut -f1)
    echo ""
    echo "Build complete!"
    echo "Binary size: $SIZE"
    echo "Location: $BINARY"
    
    # Run memory benchmark if requested
    if [ "$1" = "--benchmark" ]; then
        echo ""
        echo "Running memory benchmark..."
        "$PROJECT_ROOT/build/scripts/memory_bench.sh"
    fi
else
    echo "Error: Binary not found at $BINARY"
    exit 1
fi
