.PHONY: help version-status version-sync version-bump-patch version-bump-minor version-bump-major version-set clean clean-rust clean-tauri clean-python clean-all size

help:
	@cd rust && $(MAKE) help

version-status:
	@cd rust && $(MAKE) version-status

version-sync:
	@cd rust && $(MAKE) version-sync

version-bump-patch:
	@cd rust && $(MAKE) version-bump-patch

version-bump-minor:
	@cd rust && $(MAKE) version-bump-minor

version-bump-major:
	@cd rust && $(MAKE) version-bump-major

version-set:
	@cd rust && $(MAKE) version-set v=$(v)

# Cleanup commands
clean-rust:
	@echo "Cleaning Rust CLI build artifacts..."
	cd rust && cargo clean
	@echo "✓ Removed rust/target/ (~3GB)"

clean-tauri:
	@echo "Cleaning Tauri build artifacts..."
	cd tauri-app/src-tauri && cargo clean
	@echo "✓ Removed tauri-app/src-tauri/target/ (~5.4GB)"

clean-python:
	@echo "Cleaning Python virtual environments..."
	rm -rf venv python/venv
	@echo "✓ Removed venv/ and python/venv/ (~2.4GB)"

clean-all: clean-rust clean-tauri
	@echo ""
	@echo "✅ All build artifacts cleaned! (~8.4GB freed)"
	@echo "To rebuild: cargo build in respective directories"

size:
	@echo "Project size breakdown:"
	@du -sh . 2>/dev/null || echo "Error checking size"
	@echo ""
	@echo "Top directories:"
	@du -sh */ 2>/dev/null | sort -hr | head -10
