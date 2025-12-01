.PHONY: help version-status version-bump-patch version-bump-minor version-bump-major version-set

# Detect OS to use correct python command
ifeq ($(OS),Windows_NT)
    PYTHON := python
else
    PYTHON := python3
endif

help:
	@echo "Available commands:"
	@echo "  make version-status       - Show current version status"
	@echo "  make version-sync         - Synchronize local config"
	@echo "  make version-bump-patch   - Bump patch version (1.0.0 -> 1.0.1)"
	@echo "  make version-bump-minor   - Bump minor version (1.0.0 -> 1.1.0)"
	@echo "  make version-bump-major   - Bump major version (1.0.0 -> 2.0.0)"
	@echo "  make version-set v=1.2.3  - Set specific version"

version-status:
	@cd ../python && $(PYTHON) scripts/update_version.py status

version-sync:
	@cd ../python && $(PYTHON) scripts/update_version.py sync

version-bump-patch:
	@cd ../python && $(PYTHON) scripts/update_version.py bump --type patch

version-bump-minor:
	@cd ../python && $(PYTHON) scripts/update_version.py bump --type minor

version-bump-major:
	@cd ../python && $(PYTHON) scripts/update_version.py bump --type major

version-set:
	@if [ -z "$(v)" ]; then echo "Error: Please specify version using v=X.Y.Z"; exit 1; fi
	@cd ../python && $(PYTHON) scripts/update_version.py set $(v)
