log:
	@bat app.log

PREV_TAG := $(shell git describe --tags --abbrev=0)
RELEASE_FILE_NAME=release.md

release-draft:
	@git cliff $(PREV_TAG)..HEAD > $(RELEASE_FILE_NAME)

release-start: release-draft
	@nvim $(RELEASE_FILE_NAME)

release-template:
	@VERSION=$$(git describe --tags --abbrev=0 2>/dev/null || echo "X.Y.Z"); \
	DATE=$$(date +%Y-%m-%d); \
	printf "## $$VERSION - $$DATE\n\n### 🚀 Features\n-\n\n### 🐛 Fixes\n-\n\n### ⚡ Performance\n-\n\n### 📚 Documentation\n-\n\n### 🔧 Refactor\n-\n\n### 🔥 Breaking Changes\n- None\n\n---\nNotes:\n-" > release.md; \
	${EDITOR} release.md

release-append:
	@cat $(RELEASE_FILE_NAME) >> CHANGELOG.md

release-clean:
	@rm -f $(RELEASE_FILE_NAME)
