# CURSED Programming Language - Production Build System
# =============================================================================
# Version: 1.2.1 - Zig Implementation
# Simplified Makefile for CURSED language with Zig compiler

# Environment and Configuration
# =============================================================================
SHELL := /bin/bash
.DEFAULT_GOAL := help

# Build Configuration
ZIG_FLAGS ?=
VERBOSE ?= 0
WORKERS ?= $(shell nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

# Directories
BUILD_DIR := zig-out
OUTPUT_DIR := output

# Commands
ZIG_CMD := zig
CURSED_UNIFIED := ./zig-out/bin/cursed-zig
CURSED_ALT := ./cursed-unified

# Conditional verbosity
ifeq ($(VERBOSE),1)
    V := --verbose
    AT := 
else
    V :=
    AT := @
endif

# Colors
RESET := \033[0m
BOLD := \033[1m
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
CYAN := \033[36m

# Core Build Targets
# =============================================================================
.PHONY: all build build-release clean help dev release
.PHONY: zig-build zig-unified

all: build test ## Build and test everything

build: zig-build ## Build the CURSED Zig compiler

zig-build: ## Build the CURSED Zig compiler
	$(AT)echo -e "$(CYAN)🔧 Building CURSED Zig compiler...$(RESET)"
	$(AT)$(ZIG_CMD) build $(V) $(ZIG_FLAGS)
	$(AT)echo -e "$(GREEN)✅ Build completed$(RESET)"

zig-unified: ## Build unified CURSED compiler (alternative method)
	$(AT)echo -e "$(CYAN)🔧 Building unified CURSED compiler...$(RESET)"
	$(AT)$(ZIG_CMD) build-exe src-zig/main_unified.zig -lc --name cursed-unified $(V)
	$(AT)echo -e "$(GREEN)✅ Unified build completed$(RESET)"

build-release: ## Build optimized release version
	$(AT)echo -e "$(CYAN)🚀 Building CURSED compiler (release)...$(RESET)"
	$(AT)$(ZIG_CMD) build -Doptimize=ReleaseFast $(V) $(ZIG_FLAGS)
	$(AT)echo -e "$(GREEN)✅ Release build completed$(RESET)"

# Aliases
dev: build ## Build for development
release: build-release ## Build optimized release version

clean: ## Clean all build artifacts
	$(AT)echo -e "$(YELLOW)🧹 Cleaning build artifacts...$(RESET)"
	$(AT)rm -rf $(BUILD_DIR) $(OUTPUT_DIR)
	$(AT)rm -f cursed-unified cursed-advanced cursed-simple cursed-optimized
	$(AT)rm -rf .zig-cache
	$(AT)echo -e "$(GREEN)✅ Clean completed$(RESET)"

# Testing Framework
# =============================================================================
.PHONY: test test-all test-zig test-stdlib test-compiler test-examples
.PHONY: run-tests check verify

test: test-zig ## Run all tests

test-zig: build ## Run Zig compiler tests
	$(AT)echo -e "$(BLUE)🧪 Running Zig compiler tests...$(RESET)"
	$(AT)$(ZIG_CMD) build test $(V)

test-stdlib: build ## Test standard library modules
	$(AT)echo -e "$(BLUE)📚 Testing standard library...$(RESET)"
	$(AT)$(CURSED_UNIFIED) stdlib/comprehensive_stdlib_test.💀 $(V)

test-compiler: build ## Test compiler functionality
	$(AT)echo -e "$(BLUE)⚙️  Testing compiler functionality...$(RESET)"
	$(AT)$(CURSED_UNIFIED) basic_test.💀 $(V)
	$(AT)$(CURSED_UNIFIED) comprehensive_test.💀 $(V)

test-examples: build ## Test example programs
	$(AT)echo -e "$(BLUE)📝 Testing example programs...$(RESET)"
	$(AT)$(CURSED_UNIFIED) examples/demo.💀 $(V)

test-all: test test-stdlib test-compiler test-examples ## Run comprehensive test suite
	$(AT)echo -e "$(GREEN)✅ All tests completed$(RESET)"

# Aliases
run-tests: test ## Run standard test suite
check: test ## Quick validation of code correctness
verify: test-all ## Comprehensive validation

# CURSED Program Execution
# =============================================================================
.PHONY: run-demo run-stdlib-demo run-program
.PHONY: jit-test jit-demo performance-test

run-demo: build ## Run basic CURSED demo
	$(AT)echo -e "$(CYAN)🎯 Running CURSED demo...$(RESET)"
	$(AT)$(CURSED_UNIFIED) demo.💀 $(V)

run-stdlib-demo: build ## Run standard library demo
	$(AT)echo -e "$(CYAN)📚 Running stdlib demo...$(RESET)"
	$(AT)$(CURSED_UNIFIED) stdlib/comprehensive_stdlib_demo.💀 $(V)

run-program: build ## Run specific CURSED program (requires PROGRAM=file.💀)
ifndef PROGRAM
	$(error PROGRAM is required. Usage: make run-program PROGRAM=file.💀)
endif
	$(AT)echo -e "$(CYAN)▶️  Running CURSED program: $(PROGRAM)$(RESET)"
	$(AT)$(CURSED_UNIFIED) $(PROGRAM) $(V)

# JIT execution engine
jit-test: build ## Test JIT execution engine
	$(AT)echo -e "$(CYAN)🚀 Testing JIT execution engine...$(RESET)"
	$(AT)$(CURSED_UNIFIED) --jit comprehensive_test.💀 $(V)

jit-demo: build ## Run JIT demonstration
	$(AT)echo -e "$(CYAN)⚡ Running JIT demonstration...$(RESET)"
	$(AT)$(CURSED_UNIFIED) --jit --tier-stats demo.💀 $(V)

performance-test: build ## Run performance benchmarks
	$(AT)echo -e "$(CYAN)⏱️  Running performance benchmarks...$(RESET)"
	$(AT)$(CURSED_UNIFIED) comprehensive_performance_test.💀 $(V)

# Cross-Compilation
# =============================================================================
.PHONY: cross-compile cross-windows cross-linux cross-macos

cross-compile: ## Build for all supported platforms
	$(AT)echo -e "$(CYAN)🌐 Cross-compiling for all platforms...$(RESET)"
	$(AT)mkdir -p $(OUTPUT_DIR)/linux-x64 $(OUTPUT_DIR)/windows-x64 $(OUTPUT_DIR)/macos-x64 $(OUTPUT_DIR)/linux-arm64 $(OUTPUT_DIR)/macos-arm64
	$(AT)echo -e "$(CYAN)📦 Building Linux x64...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=x86_64-linux $(V)
	$(AT)cp zig-out/bin/* $(OUTPUT_DIR)/linux-x64/ 2>/dev/null || true
	$(AT)echo -e "$(CYAN)📦 Building Windows x64...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=x86_64-windows $(V)
	$(AT)cp zig-out/bin/* $(OUTPUT_DIR)/windows-x64/ 2>/dev/null || true
	$(AT)echo -e "$(CYAN)📦 Building macOS x64...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=x86_64-macos $(V)
	$(AT)cp zig-out/bin/* $(OUTPUT_DIR)/macos-x64/ 2>/dev/null || true
	$(AT)echo -e "$(CYAN)📦 Building Linux ARM64...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=aarch64-linux $(V)
	$(AT)cp zig-out/bin/* $(OUTPUT_DIR)/linux-arm64/ 2>/dev/null || true
	$(AT)echo -e "$(CYAN)📦 Building macOS ARM64...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=aarch64-macos $(V)
	$(AT)cp zig-out/bin/* $(OUTPUT_DIR)/macos-arm64/ 2>/dev/null || true
	$(AT)echo -e "$(GREEN)✅ Cross-compilation completed$(RESET)"
	$(AT)echo -e "$(CYAN)📂 Output directories:$(RESET)"
	$(AT)ls -la $(OUTPUT_DIR)/*/

cross-windows: ## Build Windows executable
	$(AT)echo -e "$(CYAN)🪟 Building Windows executable...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=x86_64-windows $(V)

cross-linux: ## Build Linux x86_64 executable
	$(AT)echo -e "$(CYAN)🐧 Building Linux x86_64 executable...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=x86_64-linux $(V)

cross-macos: ## Build macOS ARM64 executable
	$(AT)echo -e "$(CYAN)🍎 Building macOS ARM64 executable...$(RESET)"
	$(AT)$(ZIG_CMD) build -Dtarget=aarch64-macos $(V)

# Help System
# =============================================================================
help: ## Show this help message
	$(AT)echo -e "$(BOLD)$(CYAN)CURSED Programming Language - Build System$(RESET)"
	$(AT)echo -e "$(CYAN)============================================$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Build Commands:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; /^build|^dev|^release|^clean/ {printf "  $(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Testing Commands:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; /^test|^check|^verify/ {printf "  $(BLUE)%-20s$(RESET) %s\n", $$1, $$2}'
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)CURSED Programs:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; /^run-|^jit-|^performance/ {printf "  $(YELLOW)%-20s$(RESET) %s\n", $$1, $$2}'
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Cross-Compilation:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; /^cross-/ {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}'
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Examples:$(RESET)"
	$(AT)echo -e "  make build                     # Build CURSED compiler"
	$(AT)echo -e "  make test                      # Run all tests"
	$(AT)echo -e "  make run-demo                  # Run demo program"
	$(AT)echo -e "  make run-program PROGRAM=test.💀  # Run specific program"
	$(AT)echo -e "  make cross-compile             # Build for all platforms"
	$(AT)echo -e "  make release                   # Build optimized version"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Environment Variables:$(RESET)"
	$(AT)echo -e "  VERBOSE=1                      # Enable verbose output"
	$(AT)echo -e "  ZIG_FLAGS=\"--flag\"             # Additional Zig flags"
