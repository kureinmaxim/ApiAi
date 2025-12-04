.PHONY: version-status version-sync version-bump-patch version-bump-minor version-bump-major version-set build

# Python script path
VERSION_SCRIPT = scripts/update_version.py

# Version management commands
version-status:
	@python3 $(VERSION_SCRIPT) status

version-sync:
	@python3 $(VERSION_SCRIPT) sync

version-bump-patch:
	@python3 $(VERSION_SCRIPT) bump patch

version-bump-minor:
	@python3 $(VERSION_SCRIPT) bump minor

version-bump-major:
	@python3 $(VERSION_SCRIPT) bump major

version-set:
	@python3 $(VERSION_SCRIPT) sync $(v)

# Config management
config-backup:
	@./scripts/backup_config.sh

config-restore:
	@./scripts/restore_config.sh

# Build commands
build:
	npm run tauri build

dev:
	npm run tauri dev

clean:
	rm -rf src-tauri/target
	rm -rf node_modules

install:
	npm install

# Help
help:
	@echo "ApiAi Tauri - Version Management"
	@echo ""
	@echo "Version commands:"
	@echo "  make version-status         - Show current version"
	@echo "  make version-sync           - Sync version across all files"
	@echo "  make version-bump-patch     - Bump patch version (X.Y.Z → X.Y.Z+1)"
	@echo "  make version-bump-minor     - Bump minor version (X.Y.Z → X.Y+1.0)"
	@echo "  make version-bump-major     - Bump major version (X.Y.Z → X+1.0.0)"
	@echo "  make version-set v=X.Y.Z    - Set specific version"
	@echo ""
	@echo "Config management:"
	@echo "  make config-backup          - Backup current config"
	@echo "  make config-restore         - Restore config from template"
	@echo ""
	@echo "Build commands:"
	@echo "  make build                  - Build release"
	@echo "  make dev                    - Run in dev mode"
	@echo "  make install                - Install dependencies"
	@echo "  make clean                  - Clean build artifacts"
