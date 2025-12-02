.PHONY: help version-status version-sync version-bump-patch version-bump-minor version-bump-major version-set

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
