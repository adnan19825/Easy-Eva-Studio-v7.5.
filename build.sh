#!/bin/bash

# Hybrid KEM Build Script v0.1.0

case "$1" in
    help|"")
        echo "Hybrid KEM Build Script v0.1.0"
        echo ""
        echo "Available commands:"
        echo "  ./build.sh help      - Show this help"
        echo "  ./build.sh build     - Build project"
        echo "  ./build.sh clean     - Clean build files"
        echo "  ./build.sh test      - Run tests"
        echo "  ./build.sh backup    - Create backup"
        echo "  ./build.sh sbom      - Generate SBOM"
        echo "  ./build.sh bundle    - Create audit bundle"
        echo "  ./build.sh audit     - Full audit preparation"
        ;;
    build)
        echo "Building..."
        mkdir -p build
        cd build && cmake .. -DCMAKE_BUILD_TYPE=Release && make
        ;;
    clean)
        echo "Cleaning..."
        rm -rf build
        ;;
    test)
        ./build.sh build
        echo "Testing..."
        cd build && ./example
        ;;
    backup)
        echo "Creating backup..."
        ./scripts/quick_backup.sh
        ;;
    sbom)
        echo "Generating SBOM..."
        python3 scripts/generate_sbom.py
        ;;
    bundle)
        ./build.sh sbom
        echo "Creating audit bundle..."
        ./scripts/audit_bundle.sh
        ;;
    audit)
        ./build.sh backup
        ./build.sh sbom
        ./build.sh bundle
        echo ""
        echo "AUDIT PREPARATION COMPLETE"
        echo "=============================="
        echo "Generated files:"
        echo "  • sbom.json"
        echo "  • audit_*.tar.gz"
        echo "  • docs/AUDIT_GUIDE.md"
        echo ""
        echo "Ready for external review"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Use: ./build.sh help"
        exit 1
        ;;
esac
