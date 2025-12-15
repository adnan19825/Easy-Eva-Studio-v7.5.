help:
@echo "Hybrid KEM Makefile v0.1.0"
@echo ""
@echo "Available commands:"
@echo "  make help      - Show this help"
@echo "  make build     - Build project"
@echo "  make clean     - Clean build files"
@echo "  make test      - Run tests"
@echo "  make backup    - Create backup"
@echo "  make sbom      - Generate SBOM"
@echo "  make bundle    - Create audit bundle"
@echo "  make audit     - Full audit preparation"

build:
@echo "Building..."
@mkdir -p build
@cd build && cmake .. -DCMAKE_BUILD_TYPE=Release && make

clean:
@echo "Cleaning..."
@rm -rf build

test: build
@echo "Testing..."
@cd build && ./example

backup:
@echo "Creating backup..."
@./scripts/quick_backup.sh

sbom:
@echo "Generating SBOM..."
@python3 scripts/generate_sbom.py

bundle: sbom
@echo "Creating audit bundle..."
@./scripts/audit_bundle.sh

audit: backup sbom bundle
@echo ""
@echo "AUDIT PREPARATION COMPLETE"
@echo "=============================="
@echo "Generated files:"
@echo "  • sbom.json"
@echo "  • audit_*.tar.gz"
@echo "  • docs/AUDIT_GUIDE.md"
@echo ""
@echo "Ready for external review"
