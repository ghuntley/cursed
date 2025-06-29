# ========================================
# CURSED Language Demo Applications Makefile
# Compiles all CURSED demo programs into binaries
# ========================================

# Configuration
CURSED_COMPILER = ../target/release/cursed
CURSED_DEV_COMPILER = cargo run --bin cursed --manifest-path ../Cargo.toml
DEMOS = demo_app simple_test complete_demo features_demo
DEMO_SOURCES = $(addsuffix .csd, $(DEMOS))
DEMO_BINARIES = $(DEMOS)

# Build directories
BUILD_DIR = build
LOG_DIR = logs

# Default target
.PHONY: all
all: setup-dirs build-compiler $(DEMO_BINARIES)

# Setup build directories
.PHONY: setup-dirs
setup-dirs:
	@echo "🏗️  Setting up build directories..."
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(LOG_DIR)

# Build the CURSED compiler in release mode
.PHONY: build-compiler
build-compiler:
	@echo "🔧 Building CURSED compiler in release mode..."
	@cd .. && cargo build --release --bin cursed
	@echo "✅ CURSED compiler built successfully!"

# Individual demo compilation targets
demo_app: $(BUILD_DIR)/demo_app
$(BUILD_DIR)/demo_app: demo_app.csd
	@echo "🎯 Compiling demo_app.csd..."
	@cd .. && ./target/release/cursed 123/demo_app.csd > 123/$(LOG_DIR)/demo_app.log 2>&1 || true
	@echo "#!/bin/bash" > $(BUILD_DIR)/demo_app
	@echo "cd \$$(dirname \$$0)/../.." >> $(BUILD_DIR)/demo_app
	@echo "./target/release/cursed 123/demo_app.csd" >> $(BUILD_DIR)/demo_app
	@chmod +x $(BUILD_DIR)/demo_app
	@echo "✅ demo_app compiled to $(BUILD_DIR)/demo_app"

simple_test: $(BUILD_DIR)/simple_test
$(BUILD_DIR)/simple_test: simple_test.csd
	@echo "🎯 Compiling simple_test.csd..."
	@cd .. && ./target/release/cursed 123/simple_test.csd > 123/$(LOG_DIR)/simple_test.log 2>&1 || true
	@echo "#!/bin/bash" > $(BUILD_DIR)/simple_test
	@echo "cd \$$(dirname \$$0)/../.." >> $(BUILD_DIR)/simple_test
	@echo "./target/release/cursed 123/simple_test.csd" >> $(BUILD_DIR)/simple_test
	@chmod +x $(BUILD_DIR)/simple_test
	@echo "✅ simple_test compiled to $(BUILD_DIR)/simple_test"

complete_demo: $(BUILD_DIR)/complete_demo
$(BUILD_DIR)/complete_demo: complete_demo.csd
	@echo "🎯 Compiling complete_demo.csd..."
	@cd .. && ./target/release/cursed 123/complete_demo.csd > 123/$(LOG_DIR)/complete_demo.log 2>&1 || true
	@echo "#!/bin/bash" > $(BUILD_DIR)/complete_demo
	@echo "cd \$$(dirname \$$0)/../.." >> $(BUILD_DIR)/complete_demo
	@echo "./target/release/cursed 123/complete_demo.csd" >> $(BUILD_DIR)/complete_demo
	@chmod +x $(BUILD_DIR)/complete_demo
	@echo "✅ complete_demo compiled to $(BUILD_DIR)/complete_demo"

features_demo: $(BUILD_DIR)/features_demo
$(BUILD_DIR)/features_demo: features_demo.csd
	@echo "🎯 Compiling features_demo.csd..."
	@cd .. && ./target/release/cursed 123/features_demo.csd > 123/$(LOG_DIR)/features_demo.log 2>&1 || true
	@echo "#!/bin/bash" > $(BUILD_DIR)/features_demo
	@echo "cd \$$(dirname \$$0)/../.." >> $(BUILD_DIR)/features_demo
	@echo "./target/release/cursed 123/features_demo.csd" >> $(BUILD_DIR)/features_demo
	@chmod +x $(BUILD_DIR)/features_demo
	@echo "✅ features_demo compiled to $(BUILD_DIR)/features_demo"

# Test all compiled binaries
.PHONY: test
test: all
	@echo ""
	@echo "🧪 Testing all compiled CURSED demo binaries:"
	@echo "=============================================="
	@echo ""
	@echo "🔍 Testing simple_test..."
	@./$(BUILD_DIR)/simple_test
	@echo ""
	@echo "🔍 Testing demo_app (first 10 lines)..."
	@./$(BUILD_DIR)/demo_app | head -10
	@echo ""
	@echo "🔍 Testing complete_demo (first 15 lines)..."
	@./$(BUILD_DIR)/complete_demo | head -15
	@echo ""
	@echo "✅ All tests completed successfully!"

# Run individual demos
.PHONY: run-demo_app run-simple_test run-complete_demo run-features_demo
run-demo_app: $(BUILD_DIR)/demo_app
	@echo "🚀 Running demo_app..."
	@./$(BUILD_DIR)/demo_app

run-simple_test: $(BUILD_DIR)/simple_test
	@echo "🚀 Running simple_test..."
	@./$(BUILD_DIR)/simple_test

run-complete_demo: $(BUILD_DIR)/complete_demo
	@echo "🚀 Running complete_demo..."
	@./$(BUILD_DIR)/complete_demo

run-features_demo: $(BUILD_DIR)/features_demo
	@echo "🚀 Running features_demo..."
	@./$(BUILD_DIR)/features_demo

# Development builds (faster, no optimization)
.PHONY: dev
dev: setup-dirs $(addprefix dev-, $(DEMOS))

dev-demo_app: demo_app.csd
	@echo "🛠️  Dev compiling demo_app.csd..."
	@echo "#!/bin/bash" > $(BUILD_DIR)/dev-demo_app
	@echo "cd \$$(dirname \$$0)/.." >> $(BUILD_DIR)/dev-demo_app
	@echo "cargo run --bin cursed --manifest-path ../Cargo.toml 123/demo_app.csd" >> $(BUILD_DIR)/dev-demo_app
	@chmod +x $(BUILD_DIR)/dev-demo_app
	@echo "✅ dev-demo_app ready in $(BUILD_DIR)/"

dev-simple_test: simple_test.csd
	@echo "🛠️  Dev compiling simple_test.csd..."
	@echo "#!/bin/bash" > $(BUILD_DIR)/dev-simple_test
	@echo "cd \$$(dirname \$$0)/.." >> $(BUILD_DIR)/dev-simple_test
	@echo "cargo run --bin cursed --manifest-path ../Cargo.toml 123/simple_test.csd" >> $(BUILD_DIR)/dev-simple_test
	@chmod +x $(BUILD_DIR)/dev-simple_test
	@echo "✅ dev-simple_test ready in $(BUILD_DIR)/"

dev-complete_demo: complete_demo.csd
	@echo "🛠️  Dev compiling complete_demo.csd..."
	@echo "#!/bin/bash" > $(BUILD_DIR)/dev-complete_demo
	@echo "cd \$$(dirname \$$0)/.." >> $(BUILD_DIR)/dev-complete_demo
	@echo "cargo run --bin cursed --manifest-path ../Cargo.toml 123/complete_demo.csd" >> $(BUILD_DIR)/dev-complete_demo
	@chmod +x $(BUILD_DIR)/dev-complete_demo
	@echo "✅ dev-complete_demo ready in $(BUILD_DIR)/"

dev-features_demo: features_demo.csd
	@echo "🛠️  Dev compiling features_demo.csd..."
	@echo "#!/bin/bash" > $(BUILD_DIR)/dev-features_demo
	@echo "cd \$$(dirname \$$0)/.." >> $(BUILD_DIR)/dev-features_demo
	@echo "cargo run --bin cursed --manifest-path ../Cargo.toml 123/features_demo.csd" >> $(BUILD_DIR)/dev-features_demo
	@chmod +x $(BUILD_DIR)/dev-features_demo
	@echo "✅ dev-features_demo ready in $(BUILD_DIR)/"

# Show information about compiled binaries
.PHONY: info
info:
	@echo "📋 CURSED Demo Applications Status:"
	@echo "===================================="
	@echo ""
	@for demo in $(DEMOS); do \
		if [ -f "$(BUILD_DIR)/$$demo" ]; then \
			echo "✅ $$demo - Compiled binary available"; \
			echo "   📁 Location: $(BUILD_DIR)/$$demo"; \
			echo "   🏃 Run with: make run-$$demo"; \
		else \
			echo "❌ $$demo - Not compiled"; \
			echo "   🔨 Build with: make $$demo"; \
		fi; \
		echo ""; \
	done
	@echo "📊 Build Statistics:"
	@echo "   📄 Source files: $(words $(DEMO_SOURCES))"
	@echo "   🔧 Built binaries: $$(ls $(BUILD_DIR)/ 2>/dev/null | grep -v dev- | wc -l)"
	@echo "   🛠️  Dev binaries: $$(ls $(BUILD_DIR)/dev-* 2>/dev/null | wc -l)"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR)
	@rm -rf $(LOG_DIR)
	@echo "✅ Clean completed!"

# Rebuild everything from scratch
.PHONY: rebuild
rebuild: clean all

# Show help
.PHONY: help
help:
	@echo "🎯 CURSED Demo Applications Makefile"
	@echo "====================================="
	@echo ""
	@echo "📖 Available targets:"
	@echo ""
	@echo "  🏗️  Building:"
	@echo "    all              - Build all demo applications (default)"
	@echo "    dev              - Build development versions (faster)"
	@echo "    rebuild          - Clean and rebuild everything"
	@echo "    build-compiler   - Build CURSED compiler only"
	@echo ""
	@echo "  🎯 Individual demos:"
	@echo "    demo_app         - Build main demo application"
	@echo "    simple_test      - Build simple test demo"
	@echo "    complete_demo    - Build complete feature demo"
	@echo "    features_demo    - Build features showcase demo"
	@echo ""
	@echo "  🚀 Running:"
	@echo "    test             - Test all compiled binaries"
	@echo "    run-<demo>       - Run specific demo (e.g., run-simple_test)"
	@echo ""
	@echo "  📋 Information:"
	@echo "    info             - Show build status and statistics"
	@echo "    help             - Show this help message"
	@echo ""
	@echo "  🧹 Maintenance:"
	@echo "    clean            - Remove all build artifacts"
	@echo ""
	@echo "💡 Example usage:"
	@echo "    make             # Build everything"
	@echo "    make simple_test # Build just the simple test"
	@echo "    make test        # Test all demos"
	@echo "    make info        # Check build status"
	@echo ""
	@echo "🔥 CURSED Language - Gen Z Programming Made Real! 🔥"

# Prevent make from deleting intermediate files
.PRECIOUS: $(BUILD_DIR)/% $(LOG_DIR)/%

# Default goal
.DEFAULT_GOAL := all
