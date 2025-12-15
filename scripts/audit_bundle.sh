#!/bin/bash
echo "Creating audit bundle..."

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BUNDLE_DIR="audit_${TIMESTAMP}"
BUNDLE_FILE="${BUNDLE_DIR}.tar.gz"

mkdir -p "$BUNDLE_DIR"

echo "1. Copying source files..."
cp -r src include "$BUNDLE_DIR/" 2>/dev/null || true

echo "2. Copying documentation and config..."
cp -r docs "$BUNDLE_DIR/" 2>/dev/null || true
cp README.md CMakeLists.txt sbom.json "$BUNDLE_DIR/" 2>/dev/null || true

echo "3. Creating bundle archive..."
if tar -czf "$BUNDLE_FILE" "$BUNDLE_DIR" 2>/dev/null; then
    echo "Audit bundle created: $BUNDLE_FILE"
    rm -rf "$BUNDLE_DIR"
else
    echo "Failed to create audit bundle"
    exit 1
fi
