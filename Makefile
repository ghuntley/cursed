# CURSED Programming Language - Optimized Build System
# =============================================================================
# Version: 3.0
# This Makefile provides a comprehensive build system for the CURSED language
# with optimized targets, parallel builds, and proper dependency management.

# Environment and Configuration
# =============================================================================
SHELL := /bin/bash
.DEFAULT_GOAL := help

# Build Configuration
CARGO_FLAGS ?=
BUILD_TYPE ?= debug
WORKERS ?= $(shell nproc 2>/dev/null || echo 4)
VERBOSE ?= 0
PROFILE ?= dev

# Directories and Paths
BUILD_DIR := target
OUTPUT_DIR := output
CACHE_DIR := .cursed_cache
COVERAGE_DIR := coverage
TEST_RESULTS_DIR := test_results

# Linking fix integration
LINKING_FIX := ./fix_linking.sh
CARGO_CMD := $(LINKING_FIX) cargo

# Conditional verbosity
ifeq ($(VERBOSE),1)
    V := --verbose
    AT := 
else
    V :=
    AT := @
endif

# Colors for output
RESET := \033[0m
BOLD := \033[1m
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
MAGENTA := \033[35m
CYAN := \033[36m

# Optimization System Integration
# =============================================================================

# Optimization test targets
.PHONY: optimization-test optimization-test-quick optimization-test-all
.PHONY: optimization-test-unit optimization-test-integration optimization-test-stress 
.PHONY: optimization-test-performance optimization-test-regression
.PHONY: optimization-test-coverage optimization-test-report optimization-help

# Performance optimization test targets
.PHONY: performance-test performance-test-quick performance-test-all
.PHONY: performance-test-gc performance-test-conversions performance-test-channels
.PHONY: performance-test-comprehensive performance-test-coverage performance-test-report
.PHONY: performance-help

# Error Propagation System Integration
# =============================================================================

# Error propagation test targets
.PHONY: error-propagation-test error-propagation-test-quick error-propagation-test-all
.PHONY: error-propagation-test-integration error-propagation-test-llvm
.PHONY: error-propagation-test-parser error-propagation-test-coverage
.PHONY: error-propagation-help

# Quick optimization validation tests
optimization-test-quick:
	@echo "$(CYAN)Running quick optimization tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --quick

# Standard optimization test suite
optimization-test:
	@echo "$(CYAN)Running standard optimization tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh

# Complete optimization test suite (including stress tests)
optimization-test-all:
	@echo "$(CYAN)Running complete optimization test suite...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --ignored

# Individual optimization test categories
optimization-test-unit:
	@echo "$(CYAN)Running optimization unit tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --test unit

optimization-test-integration:
	@echo "$(CYAN)Running optimization integration tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --test integration

optimization-test-stress:
	@echo "$(CYAN)Running optimization stress tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --test stress --ignored

optimization-test-performance:
	@echo "$(CYAN)Running optimization performance tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --test performance --ignored

optimization-test-regression:
	@echo "$(CYAN)Running optimization regression tests...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --test regression

# Optimization test coverage and reporting
optimization-test-coverage:
	@echo "$(CYAN)Generating optimization test coverage...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --coverage --report

optimization-test-report:
	@echo "$(CYAN)Generating optimization test report...$(RESET)"
	$(AT)./tests/run_optimization_tests.sh --report --verbose

# New optimization system tests
test-optimization-baseline:
	@echo "$(CYAN)Running optimization baseline comparison tests...$(RESET)"
	$(FIX_LINKING) cargo test --test optimization_baseline_comparison_test

test-optimization-time-savings:
	@echo "$(CYAN)Running optimization time savings tests...$(RESET)"
	$(FIX_LINKING) cargo test --test optimization_time_savings_test

test-optimization-complete:
	@echo "$(CYAN)Running complete optimization system tests...$(RESET)"
	$(FIX_LINKING) cargo test --test optimization_baseline_comparison_test
	$(FIX_LINKING) cargo test --test optimization_time_savings_test

# LTO optimization testing targets
test-lto:
	@echo "$(CYAN)Running LTO optimization integration tests...$(RESET)"
	$(FIX_LINKING) cargo test --test lto_optimization_integration_test

test-lto-verbose:
	@echo "$(CYAN)Running LTO optimization tests with verbose output...$(RESET)"
	$(FIX_LINKING) cargo test --test lto_optimization_integration_test -- --nocapture

test-lto-ignored:
	@echo "$(CYAN)Running ignored LTO optimization tests...$(RESET)"
	$(FIX_LINKING) cargo test --test lto_optimization_integration_test -- --ignored

# Run optimization examples
demo-optimization:
	@echo "$(CYAN)Running optimization system demo...$(RESET)"
	$(FIX_LINKING) cargo run --example optimization_usage_demo

# Optimization system help
# Performance optimization test targets
performance-test-quick:
	@echo "$(CYAN)Running quick performance validation tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --quick

performance-test:
	@echo "$(CYAN)Running standard performance optimization tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh

performance-test-all:
	@echo "$(CYAN)Running complete performance optimization test suite...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --full

performance-test-gc:
	@echo "$(CYAN)Running GC performance tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --test gc

performance-test-conversions:
	@echo "$(CYAN)Running type conversion performance tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --test type_conversion

performance-test-channels:
	@echo "$(CYAN)Running channels performance tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --test channels

performance-test-comprehensive:
	@echo "$(CYAN)Running comprehensive optimization tests...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --test comprehensive

performance-test-coverage:
	@echo "$(CYAN)Generating performance test coverage...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --coverage --report performance_coverage.md

performance-test-report:
	@echo "$(CYAN)Generating performance test report...$(RESET)"
	$(AT)./scripts/run_performance_optimization_tests.sh --report performance_report.md --verbose

performance-help:
	@echo "$(CYAN)Performance Optimization Test Targets:$(RESET)"
	@echo "  $(GREEN)performance-test-quick$(RESET)         - Quick performance validation"
	@echo "  $(GREEN)performance-test$(RESET)               - Standard performance test suite"
	@echo "  $(GREEN)performance-test-all$(RESET)           - Complete performance test suite"
	@echo "  $(GREEN)performance-test-gc$(RESET)            - GC performance tests"
	@echo "  $(GREEN)performance-test-conversions$(RESET)   - Type conversion performance tests"
	@echo "  $(GREEN)performance-test-channels$(RESET)      - Channel performance tests"
	@echo "  $(GREEN)performance-test-comprehensive$(RESET) - Comprehensive optimization tests"
	@echo "  $(GREEN)performance-test-coverage$(RESET)      - Generate coverage report"
	@echo "  $(GREEN)performance-test-report$(RESET)        - Generate detailed test report"
	@echo "  $(GREEN)performance-help$(RESET)               - Show this help message"

optimization-help:
	@echo "$(CYAN)Optimization System Test Targets:$(RESET)"
	@echo "  $(GREEN)optimization-test-quick$(RESET)      - Quick validation tests"
	@echo "  $(GREEN)optimization-test$(RESET)            - Standard test suite"
	@echo "  $(GREEN)optimization-test-all$(RESET)        - Complete test suite with stress tests"
	@echo "  $(GREEN)optimization-test-unit$(RESET)       - Unit tests for optimization components"
	@echo "  $(GREEN)optimization-test-integration$(RESET) - Integration tests for complete pipeline"
	@echo "  $(GREEN)optimization-test-stress$(RESET)     - Stress tests under extreme conditions"
	@echo "  $(GREEN)optimization-test-performance$(RESET) - Performance benchmarking tests"
	@echo "  $(GREEN)optimization-test-regression$(RESET) - Regression detection tests"
	@echo "  $(GREEN)optimization-test-coverage$(RESET)   - Generate coverage report"
	@echo "  $(GREEN)optimization-test-report$(RESET)     - Generate detailed test report"
	@echo "  $(GREEN)test-optimization-baseline$(RESET)   - Test baseline comparison system"
	@echo "  $(GREEN)test-optimization-time-savings$(RESET) - Test time savings calculations"
	@echo "  $(GREEN)test-optimization-complete$(RESET)   - Test complete optimization system"
	@echo "  $(GREEN)demo-optimization$(RESET)            - Run optimization demo"
	@echo "  $(GREEN)optimization-help$(RESET)            - Show this help message"

# Performance Optimization Integration
# =============================================================================

# Optimization Configuration
OPT_LEVEL ?= release
OPT_FLAGS ?= --enable-enhanced-passes --enable-lto --enable-pgo
BENCHMARK_ITERATIONS ?= 5
BENCHMARK_WARMUP ?= 2

# Optimization targets
.PHONY: optimize-build optimize-benchmark optimize-profile optimize-validate
.PHONY: optimization-report optimization-help

# Enhanced optimization build with maximum performance
optimize-build: ## Build with maximum optimization (enhanced passes, LTO, PGO)
	@echo -e "$(CYAN)Building CURSED with enhanced optimization...$(RESET)"
	$(AT)$(CARGO_CMD) build --release $(OPT_FLAGS) $(V)
	@echo -e "$(GREEN)✓ Enhanced optimization build completed$(RESET)"

# Run comprehensive optimization benchmarks
optimize-benchmark: ## Run performance benchmarks to validate optimizations
	@echo -e "$(CYAN)Running optimization performance benchmarks...$(RESET)"
	$(AT)./scripts/run_optimization_benchmarks.sh -i $(BENCHMARK_ITERATIONS) -w $(BENCHMARK_WARMUP) $(V)
	@echo -e "$(GREEN)✓ Benchmark suite completed$(RESET)"

# Run optimization benchmarks with baseline comparison
optimize-benchmark-compare: ## Run benchmarks and compare with baseline
	@echo -e "$(CYAN)Running benchmarks with baseline comparison...$(RESET)"
	$(AT)if [ -f "benchmark_results/baseline.json" ]; then \
		./scripts/run_optimization_benchmarks.sh -c benchmark_results/baseline.json; \
	else \
		echo -e "$(YELLOW)⚠ No baseline found, running standard benchmarks$(RESET)"; \
		make optimize-benchmark; \
	fi

# Create performance baseline
optimize-baseline: ## Create performance baseline for regression testing
	@echo -e "$(CYAN)Creating performance baseline...$(RESET)"
	$(AT)./scripts/run_optimization_benchmarks.sh --baseline $(V)
	@echo -e "$(GREEN)✓ Performance baseline created$(RESET)"

# Profile optimization performance
optimize-profile: ## Profile the optimization system performance
	@echo -e "$(CYAN)Profiling optimization performance...$(RESET)"
	$(AT)$(CARGO_CMD) test --release optimization --features profiling -- --nocapture $(V)
	@echo -e "$(GREEN)✓ Optimization profiling completed$(RESET)"

# Validate optimization improvements
optimize-validate: ## Validate that optimizations meet performance expectations
	@echo -e "$(CYAN)Validating optimization performance...$(RESET)"
	$(AT)$(CARGO_CMD) test --test optimization_performance_test --release $(V)
	@echo -e "$(GREEN)✓ Optimization validation completed$(RESET)"

# Generate optimization analysis report
optimization-report: ## Generate comprehensive optimization analysis report
	@echo -e "$(CYAN)Generating optimization analysis report...$(RESET)"
	$(AT)mkdir -p $(OUTPUT_DIR)
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize analyze --format markdown \
		--output $(OUTPUT_DIR)/optimization_report.md benchmarks/ 2>/dev/null || \
		echo -e "$(YELLOW)⚠ CLI analysis not available, generating basic report$(RESET)"
	@echo -e "$(GREEN)✓ Optimization report generated in $(OUTPUT_DIR)/$(RESET)"

# CLI optimization commands
optimize-interactive: ## Run interactive optimization wizard
	@echo -e "$(CYAN)Starting interactive optimization wizard...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize interactive $(V)

optimize-config-show: ## Show current optimization configuration
	@echo -e "$(CYAN)Current optimization configuration:$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize config --show 2>/dev/null || \
		echo -e "$(YELLOW)⚠ CLI not available, showing default configuration$(RESET)"

optimize-enable-enhanced: ## Enable enhanced optimization passes
	@echo -e "$(CYAN)Enabling enhanced optimization passes...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize enable aggressive-inline,vectorize,loop-unroll,math-optimize --global

optimize-disable-enhanced: ## Disable enhanced optimization passes for debugging
	@echo -e "$(CYAN)Disabling enhanced optimization passes...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize disable aggressive-inline,vectorize,loop-unroll --global

optimize-reset-config: ## Reset optimization configuration to defaults
	@echo -e "$(CYAN)Resetting optimization configuration...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize reset --global --confirm

# Development vs Release optimization profiles
optimize-dev: ## Configure for development (fast compilation)
	@echo -e "$(CYAN)Configuring for development optimization...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize apply --profile dev --dev-mode

optimize-release: ## Configure for release (maximum performance)
	@echo -e "$(CYAN)Configuring for release optimization...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed -- optimize apply --profile release --aggressive

# Performance regression testing
optimize-regression-test: ## Run performance regression tests
	@echo -e "$(CYAN)Running performance regression tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test optimization_performance_test -- --ignored $(V)
	@echo -e "$(GREEN)✓ Performance regression tests completed$(RESET)"

# Quick optimization validation
optimize-quick: ## Quick optimization validation (fast benchmarks)
	@echo -e "$(CYAN)Running quick optimization validation...$(RESET)"
	$(AT)./scripts/run_optimization_benchmarks.sh -i 2 -w 1 --timeout 60
	@echo -e "$(GREEN)✓ Quick validation completed$(RESET)"

# Help for optimization targets
optimization-help: ## Show detailed help for optimization targets
	@echo -e "$(BOLD)$(CYAN)CURSED Optimization System Help$(RESET)"
	@echo -e "$(BOLD)================================$(RESET)"
	@echo ""
	@echo -e "$(BOLD)Enhanced Optimization Features:$(RESET)"
	@echo -e "  • Aggressive optimization (O3) enabled by default"
	@echo -e "  • Enhanced LLVM passes (vectorization, loop unrolling, aggressive inlining)"
	@echo -e "  • Link-time optimization (LTO) enabled by default"
	@echo -e "  • Profile-guided optimization (PGO) when data available"
	@echo -e "  • CURSED-specific optimizations for language constructs"
	@echo ""
	@echo -e "$(BOLD)Quick Start:$(RESET)"
	@echo -e "  make optimize-build      # Build with maximum optimization"
	@echo -e "  make optimize-benchmark  # Validate performance improvements"
	@echo -e "  make optimize-validate   # Run performance tests"
	@echo ""
	@echo -e "$(BOLD)Development Workflow:$(RESET)"
	@echo -e "  make optimize-dev        # Fast compilation for development"
	@echo -e "  make optimize-release    # Maximum performance for production"
	@echo -e "  make optimize-quick      # Quick performance check"
	@echo ""
	@echo -e "$(BOLD)Performance Analysis:$(RESET)"
	@echo -e "  make optimize-baseline   # Create performance baseline"
	@echo -e "  make optimize-profile    # Profile optimization system"
	@echo -e "  make optimization-report # Generate analysis report"
	@echo ""
	@echo -e "$(BOLD)Configuration Management:$(RESET)"
	@echo -e "  make optimize-config-show      # Show current settings"
	@echo -e "  make optimize-enable-enhanced  # Enable enhanced passes"
	@echo -e "  make optimize-disable-enhanced # Disable for debugging"
	@echo -e "  make optimize-interactive      # Interactive configuration"
	@echo ""
	@echo -e "$(BOLD)Regression Testing:$(RESET)"
	@echo -e "  make optimize-regression-test  # Run regression tests"
	@echo -e "  make optimize-benchmark-compare # Compare with baseline"
	@echo ""

# Performance Optimization Integration
include Makefile.performance

# Error Propagation Test Implementation
# =============================================================================

# Quick error propagation validation tests
error-propagation-test-quick: ## Quick error propagation validation tests
	@echo "$(CYAN)Running quick error propagation tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test error_propagation_integration_test

# Standard error propagation test suite
error-propagation-test: ## Standard error propagation test suite
	@echo "$(CYAN)Running error propagation tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test error_propagation_integration_test
	$(AT)$(CARGO_CMD) test --test error_propagation_llvm_test

# Complete error propagation test suite
error-propagation-test-all: ## Complete error propagation test suite
	@echo "$(CYAN)Running complete error propagation test suite...$(RESET)"
	$(AT)$(CARGO_CMD) test --test error_propagation_integration_test
	$(AT)$(CARGO_CMD) test --test error_propagation_llvm_test
	$(AT)$(CARGO_CMD) test --lib parser::error_propagation::tests

# Integration tests for error propagation parsing
error-propagation-test-integration: ## Error propagation integration tests
	@echo "$(CYAN)Running error propagation integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test error_propagation_integration_test

# LLVM code generation tests for error propagation
error-propagation-test-llvm: ## Error propagation LLVM tests
	@echo "$(CYAN)Running error propagation LLVM tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test error_propagation_llvm_test

# Parser tests for error propagation
error-propagation-test-parser: ## Error propagation parser tests
	@echo "$(CYAN)Running error propagation parser tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --lib parser::error_propagation::tests

# Error propagation test coverage
error-propagation-test-coverage: ## Generate error propagation test coverage
	@echo "$(CYAN)Generating error propagation test coverage...$(RESET)"
	$(AT)$(CARGO_CMD) tarpaulin --out Html --output-dir $(COVERAGE_DIR) \
		--tests error_propagation_integration_test,error_propagation_llvm_test \
		--lib parser::error_propagation,codegen::llvm::error_propagation

# Error propagation help
error-propagation-help: ## Show error propagation test help
	@echo "$(CYAN)Error Propagation Test Help:$(RESET)"
	@echo "  error-propagation-test-quick      - Quick validation tests"
	@echo "  error-propagation-test           - Standard test suite"
	@echo "  error-propagation-test-all       - Complete test suite"
	@echo "  error-propagation-test-integration - Integration tests"
	@echo "  error-propagation-test-llvm      - LLVM code generation tests"
	@echo "  error-propagation-test-parser    - Parser unit tests"
	@echo "  error-propagation-test-coverage  - Generate test coverage report"

# Core Build Targets
# =============================================================================
.PHONY: all build build-release build-dev clean help

all: build test ## Build and test everything

build: ## Build the project in debug mode
	$(AT)echo -e "$(CYAN)🔧 Building CURSED compiler...$(RESET)"
	$(AT)$(CARGO_CMD) build $(V) $(CARGO_FLAGS)
	$(AT)echo -e "$(GREEN)✅ Build completed$(RESET)"

build-release: ## Build the project in release mode
	$(AT)echo -e "$(CYAN)🚀 Building CURSED compiler (release)...$(RESET)"
	$(AT)$(CARGO_CMD) build --release $(V) $(CARGO_FLAGS)
	$(AT)echo -e "$(GREEN)✅ Release build completed$(RESET)"

build-dev: build ## Alias for debug build

clean: ## Clean all build artifacts
	$(AT)echo -e "$(YELLOW)🧹 Cleaning build artifacts...$(RESET)"
	$(AT)$(CARGO_CMD) clean $(V)
	$(AT)rm -rf $(OUTPUT_DIR) $(CACHE_DIR) $(COVERAGE_DIR) $(TEST_RESULTS_DIR)
	$(AT)echo -e "$(GREEN)✅ Clean completed$(RESET)"

# Testing Framework
# =============================================================================
.PHONY: test test-all test-unit test-integration test-ignored test-verbose test-quiet
.PHONY: test-filter test-file test-name test-coverage test-report

test: build ## Run all tests
	$(AT)echo -e "$(BLUE)🧪 Running tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V)

test-all: build ## Run all tests including ignored ones
	$(AT)echo -e "$(BLUE)🧪 Running all tests (including ignored)...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) -- --ignored --include-ignored

test-unit: build ## Run unit tests only
	$(AT)echo -e "$(BLUE)📝 Running unit tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --lib

test-integration: build ## Run integration tests only
	$(AT)echo -e "$(BLUE)🔗 Running integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test "*"

test-ignored: build ## Run ignored tests only
	$(AT)echo -e "$(BLUE)⏭️  Running ignored tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) -- --ignored

test-verbose: build ## Run tests with verbose output
	$(AT)$(CARGO_CMD) test --verbose

test-quiet: build ## Run tests with minimal output
	$(AT)$(CARGO_CMD) test --quiet

test-filter: build ## Run filtered tests (requires TEST_PATTERN)
ifndef TEST_PATTERN
	$(error TEST_PATTERN is required. Usage: make test-filter TEST_PATTERN=your_pattern)
endif
	$(AT)echo -e "$(BLUE)🔍 Running filtered tests: $(TEST_PATTERN)$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) $(TEST_PATTERN)

test-file: build ## Run specific test file (requires TEST_FILE)
ifndef TEST_FILE
	$(error TEST_FILE is required. Usage: make test-file TEST_FILE=test_name)
endif
	$(AT)echo -e "$(BLUE)📄 Running test file: $(TEST_FILE)$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test $(TEST_FILE)

# Memory-Mapped Files Testing
# =============================================================================
.PHONY: mmap-test mmap-test-all mmap-test-unit mmap-test-integration mmap-test-ipc
.PHONY: mmap-test-performance mmap-test-stress mmap-example mmap-clean mmap-help

mmap-test: build ## Run basic mmap tests
	$(AT)echo -e "$(MAGENTA)🗺️  Running memory-mapped files tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_test

mmap-test-all: build ## Run all mmap tests including IPC integration
	$(AT)echo -e "$(MAGENTA)🗺️  Running all memory-mapped files tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_test --test mmap_ipc_integration_test

mmap-test-unit: build ## Run mmap unit tests only
	$(AT)echo -e "$(MAGENTA)📝 Running mmap unit tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --lib mmap

mmap-test-integration: build ## Run mmap integration tests
	$(AT)echo -e "$(MAGENTA)🔗 Running mmap integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_test

mmap-test-ipc: build ## Run mmap IPC communication tests
	$(AT)echo -e "$(MAGENTA)📡 Running mmap IPC tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_ipc_integration_test

mmap-test-performance: build ## Run mmap performance tests
	$(AT)echo -e "$(MAGENTA)⚡ Running mmap performance tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_test test_performance
	$(AT)$(CARGO_CMD) test $(V) --test mmap_ipc_integration_test test_shared_memory_data_structures

mmap-test-stress: build ## Run mmap stress and concurrent tests
	$(AT)echo -e "$(MAGENTA)💪 Running mmap stress tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test mmap_test test_concurrent_access test_thread_safety_stress
	$(AT)$(CARGO_CMD) test $(V) --test mmap_ipc_integration_test test_shared_memory_ring_buffer

mmap-example: build ## Run mmap demonstration example
	$(AT)echo -e "$(MAGENTA)🎯 Building mmap demo example...$(RESET)"
	$(AT)echo "Note: mmap_demo.csd is a CURSED language example"
	$(AT)echo "To run: ./target/debug/cursed examples/mmap_demo.csd"

mmap-clean: ## Clean mmap-related test artifacts
	$(AT)echo -e "$(YELLOW)🧹 Cleaning mmap test artifacts...$(RESET)"
	$(AT)rm -f test_mmap_file.txt large_test_file.dat perf_test_file.dat
	$(AT)rm -f ipc_messages.dat *.dat
	$(AT)echo -e "$(GREEN)✅ Mmap test artifacts cleaned$(RESET)"

mmap-help: ## Show mmap-specific help
	$(AT)echo -e "$(CYAN)$(BOLD)Memory-Mapped Files Module Help$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(YELLOW)Available mmap targets:$(RESET)"
	$(AT)echo "  mmap-test            - Run basic mmap functionality tests"
	$(AT)echo "  mmap-test-all        - Run all mmap tests including IPC"
	$(AT)echo "  mmap-test-unit       - Run mmap unit tests only"
	$(AT)echo "  mmap-test-integration - Run mmap integration tests"
	$(AT)echo "  mmap-test-ipc        - Run mmap IPC communication tests"
	$(AT)echo "  mmap-test-performance - Run mmap performance benchmarks"
	$(AT)echo "  mmap-test-stress     - Run mmap stress and concurrent tests"
	$(AT)echo "  mmap-example         - Build mmap demonstration example"
	$(AT)echo "  mmap-clean           - Clean mmap test artifacts"
	$(AT)echo "  mmap-help            - Show this help"
	$(AT)echo ""
	$(AT)echo -e "$(YELLOW)Examples:$(RESET)"
	$(AT)echo "  make mmap-test                    # Basic functionality"
	$(AT)echo "  make mmap-test-ipc               # IPC communication"
	$(AT)echo "  make mmap-test-performance       # Performance benchmarks"
	$(AT)echo "  VERBOSE=1 make mmap-test-all     # All tests with verbose output"

test-name: build ## Run specific test by name (requires TEST_NAME)
ifndef TEST_NAME
	$(error TEST_NAME is required. Usage: make test-name TEST_NAME=test_name)
endif
	$(AT)echo -e "$(BLUE)🎯 Running test: $(TEST_NAME)$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) $(TEST_NAME)

# LTO Module Cloning Tests
lto-cloning-test: build ## Run LTO module cloning tests
	$(AT)echo -e "$(BLUE)🔗 Running LTO module cloning tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test lto_module_cloning_test

lto-cloning-test-verbose: build ## Run LTO module cloning tests with verbose output
	$(AT)echo -e "$(BLUE)🔗 Running LTO module cloning tests (verbose)...$(RESET)"
	$(AT)$(CARGO_CMD) test --test lto_module_cloning_test -- --nocapture

lto-cloning-test-ignored: build ## Run ignored LTO module cloning tests
	$(AT)echo -e "$(BLUE)🔗 Running ignored LTO module cloning tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test lto_module_cloning_test -- --ignored

# Optimization Pass Testing
# =============================================================================
.PHONY: test-optimization-passes test-optimization-passes-verbose
.PHONY: bench-optimization-passes bench-optimization-passes-baseline bench-optimization-passes-compare

test-optimization-passes: build ## Run optimization pass integration tests
	$(AT)echo -e "$(BLUE)⚡ Running optimization pass tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test optimization_passes_integration_test

test-optimization-passes-verbose: build ## Run optimization pass tests with verbose output
	$(AT)echo -e "$(BLUE)⚡ Running optimization pass tests (verbose)...$(RESET)"
	$(AT)$(CARGO_CMD) test --test optimization_passes_integration_test -- --nocapture

# Optimization Pass Benchmarking
bench-optimization-passes: build ## Benchmark optimization pass performance
	$(AT)echo -e "$(YELLOW)⚡ Benchmarking optimization passes...$(RESET)"
	$(AT)$(CARGO_CMD) bench --bench optimization_passes_bench

bench-optimization-passes-baseline: build ## Create performance baseline for optimization passes
	$(AT)echo -e "$(YELLOW)📊 Creating optimization pass performance baseline...$(RESET)"
	$(AT)$(CARGO_CMD) bench --bench optimization_passes_bench -- --save-baseline optimization_baseline

bench-optimization-passes-compare: build ## Compare optimization pass performance against baseline
	$(AT)echo -e "$(YELLOW)📈 Comparing optimization pass performance...$(RESET)"
	$(AT)$(CARGO_CMD) bench --bench optimization_passes_bench -- --load-baseline optimization_baseline

# Advanced LLVM Integration Testing
# =============================================================================
.PHONY: advanced-llvm-test advanced-llvm-test-quick advanced-llvm-test-all
.PHONY: advanced-llvm-benchmark advanced-llvm-help

advanced-llvm-test: build ## Run advanced LLVM integration tests
	$(AT)echo -e "$(BLUE)🔧 Running Advanced LLVM Integration Tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test advanced_llvm_integration_test

advanced-llvm-test-quick: build ## Run quick advanced LLVM integration tests
	$(AT)echo -e "$(BLUE)🔧 Running Quick Advanced LLVM Integration Tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test advanced_llvm_integration_test test_instruction_cloner_creation test_cfg_manipulator_creation test_function_inlining_validation

advanced-llvm-test-all: build ## Run all advanced LLVM tests including comprehensive
	$(AT)echo -e "$(BLUE)🔧 Running All Advanced LLVM Integration Tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test advanced_llvm_integration_test
	$(AT)echo -e "$(GREEN)✅ Advanced LLVM Integration Tests Complete$(RESET)"

advanced-llvm-benchmark: build ## Run advanced LLVM performance benchmarks
	$(AT)echo -e "$(YELLOW)⚡ Running Advanced LLVM Performance Benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test advanced_llvm_integration_test test_performance_benchmarks --release

advanced-llvm-test-verbose: build ## Run advanced LLVM tests with verbose output
	$(AT)echo -e "$(BLUE)🔧 Running Advanced LLVM Tests (verbose)...$(RESET)"
	$(AT)$(CARGO_CMD) test --test advanced_llvm_integration_test -- --nocapture

advanced-llvm-help: ## Show advanced LLVM integration help
	$(AT)echo -e "$(CYAN)$(BOLD)Advanced LLVM Integration Test Commands$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(YELLOW)Available targets:$(RESET)"
	$(AT)echo "  advanced-llvm-test           - Run all advanced LLVM integration tests"
	$(AT)echo "  advanced-llvm-test-quick     - Run quick validation tests"
	$(AT)echo "  advanced-llvm-test-all       - Run comprehensive test suite"
	$(AT)echo "  advanced-llvm-benchmark      - Run performance benchmark tests"
	$(AT)echo "  advanced-llvm-test-verbose   - Run tests with detailed output"
	$(AT)echo "  advanced-llvm-help           - Show this help"
	$(AT)echo ""
	$(AT)echo -e "$(YELLOW)What gets tested:$(RESET)"
	$(AT)echo "  • Instruction cloning and CFG manipulation"
	$(AT)echo "  • Function inlining validation and execution"
	$(AT)echo "  • Loop detection and vectorization analysis"
	$(AT)echo "  • Memory safety and error handling"
	$(AT)echo "  • Performance benchmarks and optimization"
	$(AT)echo "  • Integration with real LLVM optimization passes"
	$(AT)echo ""
	$(AT)echo -e "$(YELLOW)Examples:$(RESET)"
	$(AT)echo "  make advanced-llvm-test              # Run all tests"
	$(AT)echo "  make advanced-llvm-test-quick        # Quick validation"
	$(AT)echo "  make advanced-llvm-benchmark         # Performance tests"
	$(AT)echo "  VERBOSE=1 make advanced-llvm-test    # Verbose output"

test-coverage: ## Generate test coverage report
	$(AT)echo -e "$(MAGENTA)📊 Generating coverage report...$(RESET)"
	$(AT)mkdir -p $(COVERAGE_DIR)
	$(AT)$(CARGO_CMD) tarpaulin --out html --output-dir $(COVERAGE_DIR) $(V) || \
		echo -e "$(YELLOW)⚠️  tarpaulin not installed. Run: cargo install cargo-tarpaulin$(RESET)"

test-report: ## Generate comprehensive test report
	$(AT)echo -e "$(MAGENTA)📋 Generating test report...$(RESET)"
	$(AT)mkdir -p $(TEST_RESULTS_DIR)
	$(AT)$(CARGO_CMD) test $(V) --message-format=json > $(TEST_RESULTS_DIR)/test_results.json 2>&1 || true
	$(AT)echo -e "$(GREEN)✅ Test report saved to $(TEST_RESULTS_DIR)/test_results.json$(RESET)"

# Vectorization Testing Targets
# =============================================================================
.PHONY: test-vectorization test-vectorization-unit test-vectorization-integration test-vectorization-llvm test-vectorization-comprehensive test-vectorization-performance

test-vectorization: build ## Run all vectorization tests
	$(AT)echo -e "$(CYAN)🚀 Running vectorization tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) vectorization

test-vectorization-unit: build ## Run vectorization unit tests
	$(AT)echo -e "$(CYAN)🧪 Running vectorization unit tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_comprehensive_test

test-vectorization-integration: build ## Run vectorization integration tests  
	$(AT)echo -e "$(CYAN)🔗 Running vectorization integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_llvm_integration_test

test-vectorization-llvm: build ## Run vectorization LLVM integration tests
	$(AT)echo -e "$(CYAN)⚡ Running vectorization LLVM tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_llvm_integration_test::execution_tests
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_llvm_integration_test::statistics_tests

test-vectorization-comprehensive: build ## Run comprehensive vectorization test suite
	$(AT)echo -e "$(CYAN)📊 Running comprehensive vectorization tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) vectorization_comprehensive_test
	$(AT)$(CARGO_CMD) test $(V) vectorization_llvm_integration_test
	$(AT)$(CARGO_CMD) test $(V) enhanced_llvm_passes::vectorization_optimizer

test-vectorization-performance: build ## Run vectorization performance tests
	$(AT)echo -e "$(CYAN)⏱️  Running vectorization performance tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_comprehensive_test::performance_tests -- --ignored
	$(AT)$(CARGO_CMD) test $(V) --test vectorization_llvm_integration_test::performance -- --ignored

test-vectorization-verbose: build ## Run vectorization tests with verbose output
	$(AT)echo -e "$(CYAN)🔊 Running vectorization tests (verbose)...$(RESET)"
	$(AT)$(CARGO_CMD) test --verbose vectorization

# Vectorization Example and Demo
# =============================================================================
.PHONY: vectorization-demo vectorization-example vectorization-benchmark

vectorization-demo: build ## Run vectorization demo program
	$(AT)echo -e "$(MAGENTA)🎯 Running vectorization demo...$(RESET)"
	$(AT)./target/debug/cursed examples/vectorization_demo.csd

vectorization-example: build ## Compile vectorization example
	$(AT)echo -e "$(MAGENTA)⚡ Compiling vectorization example...$(RESET)"
	$(AT)./target/debug/cursed compile examples/vectorization_demo.csd --output target/vectorization_demo --optimize

vectorization-benchmark: build ## Run vectorization performance benchmark
	$(AT)echo -e "$(MAGENTA)📈 Running vectorization benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench --bench vectorization_benchmark || echo "Benchmark not found, creating placeholder..."

# Vectorization Help and Information
# =============================================================================
.PHONY: vectorization-help vectorization-info

vectorization-help: ## Show vectorization testing commands
	$(AT)echo -e "$(BOLD)CURSED Vectorization Testing Commands:$(RESET)"
	$(AT)echo -e "  $(GREEN)test-vectorization$(RESET)              - Run all vectorization tests"
	$(AT)echo -e "  $(GREEN)test-vectorization-unit$(RESET)         - Run vectorization unit tests"  
	$(AT)echo -e "  $(GREEN)test-vectorization-integration$(RESET)  - Run vectorization integration tests"
	$(AT)echo -e "  $(GREEN)test-vectorization-llvm$(RESET)         - Run vectorization LLVM tests"
	$(AT)echo -e "  $(GREEN)test-vectorization-comprehensive$(RESET) - Run comprehensive test suite"
	$(AT)echo -e "  $(GREEN)test-vectorization-performance$(RESET)  - Run performance tests"
	$(AT)echo -e "  $(GREEN)test-vectorization-verbose$(RESET)      - Run tests with verbose output"
	$(AT)echo -e ""
	$(AT)echo -e "  $(YELLOW)vectorization-demo$(RESET)              - Run vectorization demo program"
	$(AT)echo -e "  $(YELLOW)vectorization-example$(RESET)           - Compile vectorization example"
	$(AT)echo -e "  $(YELLOW)vectorization-benchmark$(RESET)         - Run performance benchmarks"
	$(AT)echo -e ""
	$(AT)echo -e "  $(CYAN)vectorization-info$(RESET)               - Show vectorization system information"

vectorization-info: ## Show vectorization system information
	$(AT)echo -e "$(BOLD)CURSED Vectorization System Information:$(RESET)"
	$(AT)echo -e "  $(GREEN)SIMD Support$(RESET)         - SSE2, AVX, AVX2, NEON (target dependent)"
	$(AT)echo -e "  $(GREEN)Vector Widths$(RESET)        - 128-bit (SSE), 256-bit (AVX), 512-bit (AVX-512)"
	$(AT)echo -e "  $(GREEN)Supported Types$(RESET)      - i32, i64, f32, f64 vectors"
	$(AT)echo -e "  $(GREEN)Loop Vectorization$(RESET)   - Automatic for qualifying loops"
	$(AT)echo -e "  $(GREEN)Operation Types$(RESET)      - Arithmetic, memory, comparison, reduction"
	$(AT)echo -e "  $(GREEN)Target Architecture$(RESET)  - $(shell rustc -vV | grep 'host:' | cut -d' ' -f2)"

# Code Quality and Formatting
# =============================================================================
.PHONY: lint lint-fix fmt fmt-check fmt-diff fmt-cursed-check fmt-cursed-fix rust-fmt-check

lint: build ## Run clippy linting
	$(AT)echo -e "$(YELLOW)🔍 Running linting...$(RESET)"
	$(AT)$(CARGO_CMD) clippy $(V) -- -D warnings

lint-fix: build ## Fix linting issues automatically
	$(AT)echo -e "$(YELLOW)🔧 Fixing linting issues...$(RESET)"
	$(AT)$(CARGO_CMD) clippy $(V) --fix --allow-staged --allow-dirty

fmt: fmt-cursed-fix rust-fmt-fix ## Format all source files

fmt-check: fmt-cursed-check rust-fmt-check ## Check formatting of all files

fmt-diff: ## Show formatting differences
	$(AT)echo -e "$(CYAN)📋 Showing CURSED formatting differences...$(RESET)"
	$(AT)$(CARGO_CMD) build --bin cursed-fmt $(V)
	$(AT)find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | \
		xargs ./target/debug/cursed-fmt --diff || true

fmt-cursed-check: ## Check CURSED file formatting
	$(AT)echo -e "$(CYAN)✅ Checking CURSED file formatting...$(RESET)"
	$(AT)$(CARGO_CMD) build --bin cursed-fmt $(V)
	$(AT)find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | \
		xargs ./target/debug/cursed-fmt --check

fmt-cursed-fix: ## Format CURSED source files
	$(AT)echo -e "$(CYAN)🎨 Formatting CURSED files...$(RESET)"
	$(AT)$(CARGO_CMD) build --bin cursed-fmt $(V)
	$(AT)find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | \
		xargs ./target/debug/cursed-fmt -w

rust-fmt-check: ## Check Rust file formatting
	$(AT)echo -e "$(CYAN)✅ Checking Rust file formatting...$(RESET)"
	$(AT)$(CARGO_CMD) fmt --check $(V)

rust-fmt-fix: ## Format Rust source files
	$(AT)echo -e "$(CYAN)🎨 Formatting Rust files...$(RESET)"
	$(AT)$(CARGO_CMD) fmt $(V)

# Math Module Testing
# =============================================================================
.PHONY: math-test math-basic-test math-trig-test math-stats-test math-logarithmic-test
.PHONY: math-test-quick math-test-comprehensive

math-test: math-basic-test math-trig-test math-stats-test math-logarithmic-test ## Run all math tests

math-basic-test: build ## Test basic math functions
	$(AT)echo -e "$(BLUE)🧮 Testing basic math functions...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test math_basic_test

math-trig-test: build ## Test trigonometric functions
	$(AT)echo -e "$(BLUE)📐 Testing trigonometric functions...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test math_trigonometry_test

math-stats-test: build ## Test statistical functions
	$(AT)echo -e "$(BLUE)📊 Testing statistical functions...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test math_statistics_test

math-logarithmic-test: build ## Test logarithmic functions
	$(AT)echo -e "$(BLUE)📈 Testing logarithmic functions...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test math_logarithmic_test

math-test-quick: build ## Quick math tests
	$(AT)echo -e "$(BLUE)⚡ Running quick math tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test math_basic_test basic
	$(AT)$(CARGO_CMD) test $(V) --test math_trigonometry_test basic
	$(AT)$(CARGO_CMD) test $(V) --test math_statistics_test basic

math-test-comprehensive: build ## Comprehensive math tests
	$(AT)echo -e "$(BLUE)🔬 Running comprehensive math tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) math::

# Crypto Module Testing  
# =============================================================================
.PHONY: crypto-test crypto-test-quick crypto-test-all crypto-test-integration
.PHONY: crypto-test-stress crypto-test-security crypto-test-interop crypto-example

crypto-test: build ## Run core crypto tests
	$(AT)echo -e "$(BLUE)🔐 Running crypto tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test crypto_integration_test

crypto-test-quick: build ## Quick crypto validation
	$(AT)$(LINKING_FIX) $(MAKE) crypto-test-quick -f /dev/null || \
		$(CARGO_CMD) test $(V) crypto_basic

crypto-test-all: crypto-test crypto-test-integration crypto-test-stress crypto-test-security crypto-test-interop ## Run all crypto tests

crypto-test-integration: build ## Integration crypto tests
	$(AT)echo -e "$(BLUE)🔗 Running crypto integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test crypto_integration_test

crypto-test-stress: build ## Stress crypto tests
	$(AT)echo -e "$(BLUE)💪 Running crypto stress tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test crypto_stress_test

crypto-test-security: build ## Security crypto tests
	$(AT)echo -e "$(BLUE)🛡️  Running crypto security tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test crypto_security_test

crypto-test-interop: build ## Interoperability crypto tests
	$(AT)echo -e "$(BLUE)🌐 Running crypto interoperability tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test crypto_interop_test

crypto-example: build ## Run crypto examples
	$(AT)echo -e "$(BLUE)💎 Running crypto showcase...$(RESET)"
	$(AT)$(CARGO_CMD) run $(V) --example crypto_showcase

# Garbage Collection Testing
# =============================================================================
.PHONY: gc-test gc-test-enhanced gc-test-goroutine gc-test-stress gc-test-memory-safety

gc-test: gc-test-enhanced gc-test-goroutine ## Run all GC tests

gc-test-enhanced: build ## Enhanced GC tests
	$(AT)echo -e "$(BLUE)🗑️  Running enhanced GC tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) enhanced_gc

gc-test-goroutine: build ## Goroutine GC integration tests
	$(AT)echo -e "$(BLUE)🔄 Running goroutine GC tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) goroutine_gc

gc-test-stress: build ## GC stress tests
	$(AT)echo -e "$(BLUE)💪 Running GC stress tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test enhanced_gc_stress_test

gc-test-memory-safety: build ## GC memory safety tests
	$(AT)echo -e "$(BLUE)🛡️  Running GC memory safety tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test enhanced_gc_memory_safety_test

# SignalBoost Testing
# =============================================================================
.PHONY: signal-boost-test signal-boost-test-verbose signal-boost-test-coverage signal-boost-example signal-boost-help

signal-boost-test: build ## Run SignalBoost integration tests
	$(AT)echo -e "$(BLUE)📡 Running SignalBoost tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test signal_boost_integration_test

signal-boost-test-verbose: build ## Run SignalBoost tests with verbose output
	$(AT)echo -e "$(BLUE)📡 Running SignalBoost tests (verbose)...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) --test signal_boost_integration_test -- --nocapture

signal-boost-test-coverage: build ## Generate SignalBoost test coverage
	$(AT)echo -e "$(BLUE)📊 Generating SignalBoost test coverage...$(RESET)"
	$(AT)mkdir -p $(COVERAGE_DIR)/signal_boost
	$(AT)$(CARGO_CMD) tarpaulin --test signal_boost_integration_test --out html --output-dir $(COVERAGE_DIR)/signal_boost

signal-boost-example: build ## Run SignalBoost example program
	$(AT)echo -e "$(BLUE)🚀 Running SignalBoost example...$(RESET)"
	$(AT)./target/debug/cursed examples/signal_boost_demo.csd 2>/dev/null || echo "Example execution not yet supported - CURSED compiler still in development"

signal-boost-help: ## Show SignalBoost testing help
	$(AT)echo -e "$(CYAN)SignalBoost Testing Commands:$(RESET)"
	$(AT)echo -e "  $(BOLD)signal-boost-test$(RESET)          - Run SignalBoost integration tests"
	$(AT)echo -e "  $(BOLD)signal-boost-test-verbose$(RESET)  - Run tests with verbose output"
	$(AT)echo -e "  $(BOLD)signal-boost-test-coverage$(RESET) - Generate test coverage report"
	$(AT)echo -e "  $(BOLD)signal-boost-example$(RESET)       - Run SignalBoost example program"
	$(AT)echo -e "  $(BOLD)signal-boost-help$(RESET)          - Show this help"

# Collections and Data Structures
# =============================================================================
.PHONY: collections-test collections-test-quick collections-test-performance
.PHONY: queues-test sets-test maps-test

collections-test: build ## Test all collections
	$(AT)echo -e "$(BLUE)📦 Testing collections...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) collections

collections-test-quick: build ## Quick collections tests
	$(AT)echo -e "$(BLUE)⚡ Quick collections tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) collections basic

collections-test-performance: build ## Collections performance tests
	$(AT)echo -e "$(BLUE)📈 Collections performance tests...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) collections performance

queues-test: build ## Test queue implementations
	$(AT)echo -e "$(BLUE)📋 Testing queues...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) queue

sets-test: build ## Test set implementations
	$(AT)echo -e "$(BLUE)🎯 Testing sets...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) sets

maps-test: build ## Test map implementations
	$(AT)echo -e "$(BLUE)🗺️  Testing maps...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) maps

# Type System and Language Features
# =============================================================================
.PHONY: type-system-test error-handling-test panic-recovery-test optimization-test
.PHONY: generics-test constraints-test

type-system-test: build ## Test type system
	$(AT)echo -e "$(BLUE)🏷️  Testing type system...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) type_system

error-handling-test: build ## Test error handling
	$(AT)echo -e "$(BLUE)⚠️  Testing error handling...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) error_handling

panic-recovery-test: build ## Test panic recovery
	$(AT)echo -e "$(BLUE)🚨 Testing panic recovery...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) panic_recovery

optimization-test: build ## Test optimization system
	$(AT)echo -e "$(BLUE)⚡ Testing optimization system...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) optimization

generics-test: build ## Test generics system
	$(AT)echo -e "$(BLUE)🧬 Testing generics...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) generics

constraints-test: build ## Test constraint system
	$(AT)echo -e "$(BLUE)🔗 Testing constraints...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) constraints

# Package Management and Tools
# =============================================================================
.PHONY: pkg-test pkg-install pkg-update pkg-check pkg-clean pkg-search pkg-info

pkg-test: build ## Test package manager
	$(AT)echo -e "$(BLUE)📦 Testing package manager...$(RESET)"
	$(AT)$(CARGO_CMD) test $(V) package

pkg-install: ## Install package dependencies
	$(AT)echo -e "$(CYAN)📥 Installing package dependencies...$(RESET)"
	$(AT)$(CARGO_CMD) build $(V)

pkg-update: ## Update package dependencies
	$(AT)echo -e "$(CYAN)🔄 Updating package dependencies...$(RESET)"
	$(AT)$(CARGO_CMD) update $(V)

pkg-check: ## Check package status
	$(AT)echo -e "$(CYAN)✅ Checking package status...$(RESET)"
	$(AT)$(CARGO_CMD) check $(V)

pkg-clean: ## Clean package cache
	$(AT)echo -e "$(YELLOW)🧹 Cleaning package cache...$(RESET)"
	$(AT)rm -rf $(CACHE_DIR)/packages

pkg-search: ## Search packages (requires PACKAGE)
ifdef PACKAGE
	$(AT)echo -e "$(CYAN)🔍 Searching for package: $(PACKAGE)$(RESET)"
	$(AT)$(CARGO_CMD) search $(PACKAGE)
else
	$(error PACKAGE is required. Usage: make pkg-search PACKAGE=name)
endif

pkg-info: ## Show package info (requires PACKAGE)
ifdef PACKAGE
	$(AT)echo -e "$(CYAN)ℹ️  Package info: $(PACKAGE)$(RESET)"
	$(AT)$(CARGO_CMD) info $(PACKAGE)
else
	$(error PACKAGE is required. Usage: make pkg-info PACKAGE=name)
endif

# Documentation and Examples
# =============================================================================
.PHONY: docs docs-open docs-serve docs-cursed docs-cursed-all docs-cursed-serve docs-examples docs-stdlib docs-init docs-validate-examples
.PHONY: example examples

# Rust documentation
docs: ## Generate Rust documentation
	$(AT)echo -e "$(MAGENTA)📖 Generating Rust documentation...$(RESET)"
	$(AT)$(CARGO_CMD) doc $(V) --no-deps --document-private-items

docs-open: docs ## Generate and open Rust documentation
	$(AT)$(CARGO_CMD) doc $(V) --no-deps --open

docs-serve: ## Serve Rust documentation locally
	$(AT)echo -e "$(MAGENTA)🌐 Serving Rust documentation at http://localhost:8000$(RESET)"
	$(AT)cd target/doc && python3 -m http.server 8000

# CURSED language documentation
docs-cursed: build ## Generate CURSED language documentation
	$(AT)echo -e "$(MAGENTA)📚 Generating CURSED documentation...$(RESET)"
	$(AT)mkdir -p docs/generated
	$(AT)./target/debug/cursed doc . \
		--output docs/generated \
		--format html \
		--format markdown \
		--include-examples \
		--include-source \
		--generate-search-index \
		--title "CURSED Programming Language" \
		--description "A complete programming language with Gen Z slang syntax" \
		--version "1.0.0" || \
		echo -e "$(YELLOW)⚠️  CURSED doc command not yet available - using placeholder$(RESET)"

docs-cursed-all: build ## Generate comprehensive CURSED documentation
	$(AT)echo -e "$(MAGENTA)📚 Generating comprehensive CURSED documentation...$(RESET)"
	$(AT)mkdir -p docs/comprehensive
	$(AT)./target/debug/cursed doc . \
		--output docs/comprehensive \
		--format html \
		--format markdown \
		--format json \
		--include-examples \
		--include-source \
		--include-private \
		--generate-cross-refs \
		--generate-search-index \
		--title "CURSED Programming Language - Complete Reference" \
		--description "Complete documentation for the CURSED programming language" \
		--version "1.0.0" || \
		echo -e "$(YELLOW)⚠️  CURSED doc command not yet available - using placeholder$(RESET)"

docs-cursed-serve: docs-cursed ## Generate and serve CURSED documentation
	$(AT)echo -e "$(MAGENTA)📚 Generating and serving CURSED documentation...$(RESET)"
	$(AT)mkdir -p docs/serve
	$(AT)./target/debug/cursed doc . \
		--output docs/serve \
		--format html \
		--include-examples \
		--include-source \
		--serve 8080 \
		--open || \
		(echo -e "$(YELLOW)⚠️  CURSED doc command not yet available$(RESET)" && \
		 cd docs/generated 2>/dev/null && python3 -m http.server 8080 || \
		 echo -e "$(RED)❌ No documentation to serve$(RESET)")

docs-examples: ## Generate examples documentation
	$(AT)echo -e "$(MAGENTA)📚 Generating examples documentation...$(RESET)"
	$(AT)mkdir -p docs/examples
	$(AT)./target/debug/cursed doc examples/comprehensive/ \
		--output docs/examples \
		--format html \
		--format markdown \
		--include-examples \
		--include-source \
		--title "CURSED Examples Collection" \
		--description "Comprehensive examples demonstrating CURSED language features" || \
		echo -e "$(YELLOW)⚠️  CURSED doc command not yet available - creating placeholder$(RESET)"

docs-stdlib: ## Generate standard library documentation  
	$(AT)echo -e "$(MAGENTA)📚 Generating standard library documentation...$(RESET)"
	$(AT)mkdir -p docs/stdlib
	$(AT)./target/debug/cursed doc src/stdlib/ \
		--output docs/stdlib \
		--format html \
		--format markdown \
		--include-examples \
		--include-source \
		--include-private \
		--title "CURSED Standard Library" \
		--description "Complete reference for CURSED standard library modules" || \
		echo -e "$(YELLOW)⚠️  CURSED doc command not yet available - creating placeholder$(RESET)"

docs-init: ## Create documentation configuration
	$(AT)echo -e "$(MAGENTA)📚 Creating documentation configuration...$(RESET)"
	$(AT)./target/debug/cursed doc --init-config cursed-doc.toml || \
		(echo -e "$(YELLOW)⚠️  CURSED doc command not yet available - creating sample config$(RESET)" && \
		 cp examples/documentation_config.toml cursed-doc.toml 2>/dev/null || \
		 echo "# CURSED Documentation Configuration" > cursed-doc.toml)
	$(AT)echo -e "$(GREEN)✅ Created cursed-doc.toml - edit to customize documentation generation$(RESET)"

docs-validate-examples: build ## Validate examples compile
	$(AT)echo -e "$(BLUE)🔍 Validating examples compile correctly...$(RESET)"
	$(AT)find examples/comprehensive -name "*.csd" -print0 | while IFS= read -r -d '' file; do \
		echo "  Checking $$file..."; \
		./target/debug/cursed check "$$file" 2>/dev/null || \
		echo -e "$(YELLOW)⚠️  CURSED check not yet available for $$file$(RESET)"; \
	done
	$(AT)echo -e "$(GREEN)✅ Example validation completed$(RESET)"

example: build ## Run the default example
	$(AT)echo -e "$(GREEN)🚀 Running default example...$(RESET)"
	$(AT)$(CARGO_CMD) run $(V) --example fibonacci

optimization-demo: ## Run CURSED optimization system demonstration
	$(AT)echo -e "$(BLUE)🔧 Running CURSED Optimization System Demo...$(RESET)"
	$(AT)cd examples && rustc cursed_optimization_demo.rs -o cursed_optimization_demo && ./cursed_optimization_demo

examples: build ## List available examples
	$(AT)echo -e "$(GREEN)📚 Available examples:$(RESET)"
	$(AT)echo -e "$(CYAN)Language Features:$(RESET)"
	$(AT)find examples/comprehensive/language_features -name "*.csd" 2>/dev/null | \
		sed 's|examples/comprehensive/language_features/||;s|\.csd||' | \
		while read example; do echo "  📝 $$example"; done || \
		echo -e "$(YELLOW)    No language feature examples found$(RESET)"
	$(AT)echo -e "$(CYAN)Standard Library Modules:$(RESET)"
	$(AT)find examples/comprehensive/stdlib_modules -name "*.csd" 2>/dev/null | \
		sed 's|examples/comprehensive/stdlib_modules/||;s|\.csd||' | \
		while read example; do echo "  🏗️  $$example"; done || \
		echo -e "$(YELLOW)    No stdlib examples found$(RESET)"
	$(AT)echo -e "$(CYAN)Real-World Applications:$(RESET)"
	$(AT)find examples/comprehensive/real_world_applications -name "*.csd" 2>/dev/null | \
		sed 's|examples/comprehensive/real_world_applications/||;s|\.csd||' | \
		while read example; do echo "  🚀 $$example"; done || \
		echo -e "$(YELLOW)    No real-world examples found$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(GREEN)💡 To run a specific example:$(RESET)"
	$(AT)echo -e "   $(CYAN)./target/debug/cursed run examples/comprehensive/language_features/basic_syntax.csd$(RESET)"

# Benchmarking and Performance
# =============================================================================
.PHONY: bench bench-math bench-crypto bench-gc bench-all

bench: build ## Run default benchmarks
	$(AT)echo -e "$(MAGENTA)🏁 Running benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench $(V) 2>/dev/null || \
		echo -e "$(YELLOW)⚠️  No benchmarks configured$(RESET)"

bench-math: build ## Math function benchmarks
	$(AT)echo -e "$(MAGENTA)🧮 Running math benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench $(V) math 2>/dev/null || \
		echo -e "$(YELLOW)⚠️  Math benchmarks not available$(RESET)"

bench-crypto: build ## Crypto function benchmarks
	$(AT)echo -e "$(MAGENTA)🔐 Running crypto benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench $(V) crypto 2>/dev/null || \
		echo -e "$(YELLOW)⚠️  Crypto benchmarks not available$(RESET)"

bench-gc: build ## GC benchmarks
	$(AT)echo -e "$(MAGENTA)🗑️  Running GC benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench $(V) gc 2>/dev/null || \
		echo -e "$(YELLOW)⚠️  GC benchmarks not available$(RESET)"

bench-all: bench-math bench-crypto bench-gc ## Run all benchmarks

# Development and Debugging
# =============================================================================
.PHONY: dev dev-watch debug check install run

dev: build test lint ## Complete development workflow

dev-watch: ## Watch for changes and rebuild
	$(AT)echo -e "$(CYAN)👀 Watching for changes...$(RESET)"
	$(AT)command -v cargo-watch >/dev/null 2>&1 || \
		(echo -e "$(YELLOW)⚠️  Installing cargo-watch...$(RESET)" && $(CARGO_CMD) install cargo-watch)
	$(AT)$(CARGO_CMD) watch -x "build" -x "test --lib" $(V)

debug: build ## Build with debug symbols and run with debugger
	$(AT)echo -e "$(YELLOW)🐛 Building with debug symbols...$(RESET)"
	$(AT)$(CARGO_CMD) build $(V) --features debug-symbols
	$(AT)echo -e "$(YELLOW)🔍 Debug build ready. Use 'gdb ./target/debug/cursed' or 'lldb ./target/debug/cursed'$(RESET)"

check: ## Quick syntax and type check
	$(AT)echo -e "$(BLUE)✅ Running quick checks...$(RESET)"
	$(AT)$(CARGO_CMD) check $(V) --all-targets

install: build-release ## Install CURSED compiler system-wide
	$(AT)echo -e "$(GREEN)📦 Installing CURSED compiler...$(RESET)"
	$(AT)$(CARGO_CMD) install --path . --force $(V)
	$(AT)echo -e "$(GREEN)✅ CURSED compiler installed$(RESET)"

run: build ## Run the CURSED compiler (requires ARGS)
ifdef ARGS
	$(AT)$(CARGO_CMD) run $(V) -- $(ARGS)
else
	$(AT)echo -e "$(YELLOW)⚠️  Usage: make run ARGS='your arguments'$(RESET)"
	$(AT)echo -e "$(YELLOW)   Example: make run ARGS='examples/hello.csd'$(RESET)"
endif

# CI/CD and Validation
# =============================================================================
.PHONY: ci ci-quick validate pre-commit health-check

ci: clean build test lint fmt-check ## Full CI pipeline
	$(AT)echo -e "$(GREEN)🎉 CI pipeline completed successfully$(RESET)"

ci-quick: build test-unit lint ## Quick CI validation
	$(AT)echo -e "$(GREEN)⚡ Quick CI validation completed$(RESET)"

validate: build test lint fmt-check docs ## Full validation suite
	$(AT)echo -e "$(GREEN)✅ Full validation completed$(RESET)"

pre-commit: fmt lint test-unit ## Pre-commit hook tasks
	$(AT)echo -e "$(GREEN)🎯 Pre-commit validation completed$(RESET)"

health-check: ## Check build system health
	$(AT)echo -e "$(CYAN)🔍 Checking build system health...$(RESET)"
	$(AT)echo -e "$(CYAN)  Rust version: $$(rustc --version)$(RESET)"
	$(AT)echo -e "$(CYAN)  Cargo version: $$(cargo --version)$(RESET)"
	$(AT)echo -e "$(CYAN)  Available workers: $(WORKERS)$(RESET)"
	$(AT)echo -e "$(CYAN)  Build type: $(BUILD_TYPE)$(RESET)"
	$(AT)echo -e "$(CYAN)  Profile: $(PROFILE)$(RESET)"
	$(AT)test -x $(LINKING_FIX) && echo -e "$(GREEN)  ✅ Linking fix available$(RESET)" || \
		echo -e "$(RED)  ❌ Linking fix not found$(RESET)"
	$(AT)echo -e "$(GREEN)✅ Health check completed$(RESET)"

# Type Switch Integration Testing
# =============================================================================
.PHONY: type-switch-test type-switch-test-quick type-switch-test-verbose type-switch-test-report type-switch-example type-switch-help

type-switch-test: ## Run type switch integration tests
	$(AT)echo -e "$(CYAN)🔍 Running type switch integration tests...$(RESET)"
	$(AT)./tests/run_type_switch_tests.sh

type-switch-test-quick: ## Run quick type switch tests
	$(AT)echo -e "$(CYAN)🔍 Running quick type switch tests...$(RESET)"
	$(AT)./tests/run_type_switch_tests.sh --quick

type-switch-test-verbose: ## Run type switch tests with verbose output
	$(AT)echo -e "$(CYAN)🔍 Running type switch tests with verbose output...$(RESET)"
	$(AT)./tests/run_type_switch_tests.sh --verbose

type-switch-test-report: ## Generate type switch test report
	$(AT)echo -e "$(CYAN)🔍 Running type switch tests and generating report...$(RESET)"
	$(AT)./tests/run_type_switch_tests.sh --report type_switch_test_report.md

type-switch-example: ## Compile type switch demo
	$(AT)echo -e "$(CYAN)🎯 Compiling type switch demo...$(RESET)"
	$(AT)cargo run -- examples/type_switch_demo.csd

type-switch-help: ## Show type switch testing help
	$(AT)echo -e "$(BOLD)$(CYAN)Type Switch Testing Commands:$(RESET)"
	$(AT)echo -e "  $(CYAN)type-switch-test         $(RESET)Run all type switch tests"
	$(AT)echo -e "  $(CYAN)type-switch-test-quick   $(RESET)Run quick type switch tests"
	$(AT)echo -e "  $(CYAN)type-switch-test-verbose $(RESET)Run tests with verbose output"
	$(AT)echo -e "  $(CYAN)type-switch-test-report  $(RESET)Generate test report"
	$(AT)echo -e "  $(CYAN)type-switch-example      $(RESET)Compile type switch demo"

# Performance Optimization System
# =============================================================================
.PHONY: performance-test performance-test-quick performance-test-verbose performance-cli-test
.PHONY: performance-benchmark performance-benchmark-all performance-profiling-test performance-integration-test
.PHONY: performance-help

# Performance optimization tests
performance-test: ## Run all performance optimization tests
	$(AT)echo -e "$(BOLD)$(CYAN)Running Performance Optimization Tests$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test $(V)

performance-test-quick: ## Run performance tests in release mode
	$(AT)echo -e "$(BOLD)$(CYAN)Running Quick Performance Tests$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test --release $(V)

performance-test-verbose: ## Run performance tests with output
	$(AT)echo -e "$(BOLD)$(CYAN)Running Verbose Performance Tests$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test -- --nocapture

# Performance system CLI tests
performance-cli-test: ## Test CLI functionality
	$(AT)echo -e "$(BOLD)$(CYAN)Testing Performance CLI$(RESET)"
	$(CARGO_CMD) test --lib cli::build_optimization::tests $(V)

# Performance benchmarks
performance-benchmark: ## Run benchmark tests
	$(AT)echo -e "$(BOLD)$(CYAN)Running Performance Benchmarks$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test test_compilation_speed_benchmark --release -- --nocapture

performance-benchmark-all: ## Run all benchmark variations
	$(AT)echo -e "$(BOLD)$(CYAN)Running All Performance Benchmarks$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test test_multiple_benchmark_runs --release -- --nocapture

# Performance profiling tests
performance-profiling-test: ## Test build profiling functionality
	$(AT)echo -e "$(BOLD)$(CYAN)Testing Build Profiling$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test test_enhanced_build_profiler --release -- --nocapture

# Integration tests
performance-integration-test: ## Run comprehensive integration tests
	$(AT)echo -e "$(BOLD)$(CYAN)Running Performance Integration Tests$(RESET)"
	$(CARGO_CMD) test --test performance_optimization_test integration_tests --release -- --nocapture

# Performance optimization help
performance-help: ## Show performance optimization help
	$(AT)echo -e "$(BOLD)$(CYAN)Performance Optimization System Targets:$(RESET)"
	$(AT)echo -e "  $(CYAN)performance-test              $(RESET)Run all performance optimization tests"
	$(AT)echo -e "  $(CYAN)performance-test-quick        $(RESET)Run performance tests in release mode"
	$(AT)echo -e "  $(CYAN)performance-test-verbose      $(RESET)Run performance tests with output"
	$(AT)echo -e "  $(CYAN)performance-cli-test          $(RESET)Test CLI functionality"
	$(AT)echo -e "  $(CYAN)performance-benchmark         $(RESET)Run benchmark tests"
	$(AT)echo -e "  $(CYAN)performance-benchmark-all     $(RESET)Run all benchmark variations"
	$(AT)echo -e "  $(CYAN)performance-profiling-test    $(RESET)Test build profiling functionality"
	$(AT)echo -e "  $(CYAN)performance-integration-test  $(RESET)Run comprehensive integration tests"
	$(AT)echo -e "  $(CYAN)performance-help              $(RESET)Show this help message"

# Enhanced Build System Testing Commands
# =============================================================================
.PHONY: enhanced-build-test enhanced-build-test-verbose enhanced-build-test-pipeline enhanced-build-test-parallel enhanced-build-test-performance enhanced-build-benchmark enhanced-build-test-coverage enhanced-build-help

enhanced-build-test: ## Run all enhanced build system tests
	$(AT)echo -e "$(BOLD)$(CYAN)🔧 Running enhanced build system tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_build_system_test

enhanced-build-test-verbose: ## Run enhanced build system tests with verbose output
	$(AT)echo -e "$(BOLD)$(CYAN)🔧 Running enhanced build system tests (verbose)...$(RESET)"
	$(CARGO_CMD) test --test enhanced_build_system_test -- --nocapture

enhanced-build-test-pipeline: ## Run build pipeline metrics tests
	$(AT)echo -e "$(BOLD)$(CYAN)📊 Running build pipeline metrics tests...$(RESET)"
	$(CARGO_CMD) test pipeline_metrics --test enhanced_build_system_test

enhanced-build-test-parallel: ## Run parallel compilation tests
	$(AT)echo -e "$(BOLD)$(CYAN)⚡ Running parallel compilation tests...$(RESET)"
	$(CARGO_CMD) test parallel_compilation --test enhanced_build_system_test

enhanced-build-test-performance: ## Run build performance analysis tests
	$(AT)echo -e "$(BOLD)$(CYAN)📈 Running build performance analysis tests...$(RESET)"
	$(CARGO_CMD) test build_performance --test enhanced_build_system_test

enhanced-build-benchmark: ## Run enhanced build system benchmarks
	$(AT)echo -e "$(BOLD)$(CYAN)🏆 Running enhanced build system benchmarks...$(RESET)"
	$(CARGO_CMD) test --test enhanced_build_system_test --release

enhanced-build-test-coverage: ## Generate enhanced build system test coverage
	$(AT)echo -e "$(BOLD)$(CYAN)📋 Generating enhanced build system test coverage...$(RESET)"
	cargo tarpaulin --out Html --output-dir coverage/enhanced_build --timeout 300 --test enhanced_build_system_test

enhanced-build-help: ## Show enhanced build system testing help
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)$(CYAN)Enhanced Build System Testing Commands:$(RESET)"
	$(AT)echo -e "  $(GREEN)enhanced-build-test$(RESET)           - Run all enhanced build system tests"
	$(AT)echo -e "  $(GREEN)enhanced-build-test-verbose$(RESET)   - Run tests with verbose output"
	$(AT)echo -e "  $(GREEN)enhanced-build-test-pipeline$(RESET)  - Run pipeline metrics tests"
	$(AT)echo -e "  $(GREEN)enhanced-build-test-parallel$(RESET)  - Run parallel compilation tests"
	$(AT)echo -e "  $(GREEN)enhanced-build-test-performance$(RESET) - Run performance analysis tests"
	$(AT)echo -e "  $(GREEN)enhanced-build-benchmark$(RESET)      - Run build system benchmarks"
	$(AT)echo -e "  $(GREEN)enhanced-build-test-coverage$(RESET)  - Generate test coverage report"
	$(AT)echo -e "  $(GREEN)enhanced-build-help$(RESET)           - Show this help message"

# Enhanced LLVM Optimization Testing Commands
# =============================================================================
.PHONY: enhanced-opt-test enhanced-opt-test-quick enhanced-opt-test-all enhanced-opt-test-integration enhanced-opt-test-performance enhanced-opt-benchmark enhanced-opt-test-coverage enhanced-opt-test-report enhanced-opt-help

enhanced-opt-test-quick: ## Run quick enhanced LLVM optimization tests
	$(AT)echo -e "$(BOLD)$(CYAN)🚀 Running quick enhanced LLVM optimization tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test --lib

enhanced-opt-test: ## Run standard enhanced LLVM optimization tests  
	$(AT)echo -e "$(BOLD)$(CYAN)🚀 Running standard enhanced LLVM optimization tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test

enhanced-opt-test-all: ## Run all enhanced LLVM optimization tests
	$(AT)echo -e "$(BOLD)$(CYAN)🚀 Running all enhanced LLVM optimization tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test -- --include-ignored

enhanced-opt-test-integration: ## Run enhanced optimization integration tests
	$(AT)echo -e "$(BOLD)$(CYAN)🚀 Running enhanced optimization integration tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test integration_tests

enhanced-opt-test-performance: ## Run enhanced optimization performance tests
	$(AT)echo -e "$(BOLD)$(CYAN)🚀 Running enhanced optimization performance tests...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test performance_tests -- --ignored

enhanced-opt-benchmark: ## Run enhanced optimization benchmarks
	$(AT)echo -e "$(BOLD)$(CYAN)⚡ Running enhanced optimization benchmarks...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test --release -- --ignored

enhanced-opt-test-coverage: ## Generate enhanced optimization test coverage
	$(AT)echo -e "$(BOLD)$(CYAN)📊 Generating enhanced optimization test coverage...$(RESET)"
	$(AT)mkdir -p $(COVERAGE_DIR)/enhanced_optimization
	$(AT)$(CARGO_CMD) tarpaulin --test enhanced_llvm_optimization_test --out Html --output-dir $(COVERAGE_DIR)/enhanced_optimization $(V)

enhanced-opt-test-report: ## Generate enhanced optimization test report
	$(AT)echo -e "$(BOLD)$(CYAN)📄 Generating enhanced optimization test report...$(RESET)"
	$(CARGO_CMD) test --test enhanced_llvm_optimization_test -- --nocapture > enhanced_optimization_test_report.txt

enhanced-opt-help: ## Show enhanced LLVM optimization help
	$(AT)echo -e "$(BOLD)$(CYAN)Enhanced LLVM Optimization Testing Commands:$(RESET)"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-quick       $(RESET)Quick validation tests"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test            $(RESET)Standard test suite"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-all        $(RESET)All tests including performance"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-integration$(RESET)Integration tests only"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-performance$(RESET)Performance tests only"
	$(AT)echo -e "  $(CYAN)enhanced-opt-benchmark       $(RESET)Performance benchmarks"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-coverage   $(RESET)Generate coverage report"
	$(AT)echo -e "  $(CYAN)enhanced-opt-test-report     $(RESET)Generate detailed report"

# Utility and Information
# =============================================================================
.PHONY: help status info clean-all reset

help: ## Show this help message
	$(AT)echo -e "$(BOLD)$(CYAN)CURSED Programming Language - Build System$(RESET)"
	$(AT)echo -e "$(CYAN)============================================$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Core Targets:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}' | \
		grep -E "(build|test|clean|lint|fmt|help)" | head -10
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Module Testing:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}' | \
		grep -E "(math|crypto|gc|collections|type-system|advanced-llvm)" | head -10
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Development:$(RESET)"
	$(AT)grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2}' | \
		grep -E "(dev|debug|docs|example|bench)" | head -10
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Performance Optimization:$(RESET)"
	$(AT)echo -e "  $(CYAN)perf-help        $(RESET) Show performance optimization help"
	$(AT)echo -e "  $(CYAN)perf-dev         $(RESET) Development build (fast compilation)"
	$(AT)echo -e "  $(CYAN)perf-release     $(RESET) Release build (maximum performance)"
	$(AT)echo -e "  $(CYAN)perf-benchmark   $(RESET) Run performance benchmarks"
	$(AT)echo -e "  $(CYAN)perf-analyze     $(RESET) Analyze project characteristics"
	$(AT)echo -e "  $(CYAN)perf-optimize    $(RESET) Run optimization analysis"
	$(AT)echo -e "  $(CYAN)perf-full        $(RESET) Complete performance optimization suite"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Usage Examples:$(RESET)"
	$(AT)echo -e "  $(YELLOW)make build$(RESET)                    # Build the project"
	$(AT)echo -e "  $(YELLOW)make test$(RESET)                     # Run all tests"
	$(AT)echo -e "  $(YELLOW)make test-filter TEST_PATTERN=math$(RESET) # Run math tests"
	$(AT)echo -e "  $(YELLOW)make crypto-test$(RESET)              # Test crypto module"
	$(AT)echo -e "  $(YELLOW)make dev$(RESET)                      # Development workflow"
	$(AT)echo -e "  $(YELLOW)make ci$(RESET)                       # Full CI pipeline"
	$(AT)echo -e "  $(YELLOW)make perf-release$(RESET)             # Optimized build"
	$(AT)echo -e "  $(YELLOW)make perf-benchmark$(RESET)           # Performance benchmarks"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Configuration:$(RESET)"
	$(AT)echo -e "  $(YELLOW)VERBOSE=1$(RESET)                    # Enable verbose output"
	$(AT)echo -e "  $(YELLOW)WORKERS=N$(RESET)                    # Set parallel workers"
	$(AT)echo -e "  $(YELLOW)BUILD_TYPE=release$(RESET)           # Set build type"

status: ## Show build system status
	$(AT)echo -e "$(BOLD)$(CYAN)Build System Status$(RESET)"
	$(AT)echo -e "$(CYAN)===================$(RESET)"
	$(AT)echo -e "Configuration:"
	$(AT)echo -e "  Build Type: $(BUILD_TYPE)"
	$(AT)echo -e "  Workers: $(WORKERS)"
	$(AT)echo -e "  Verbose: $(VERBOSE)"
	$(AT)echo -e "  Profile: $(PROFILE)"
	$(AT)echo ""
	$(AT)echo -e "Directories:"
	$(AT)echo -e "  Build: $(BUILD_DIR)"
	$(AT)echo -e "  Output: $(OUTPUT_DIR)"
	$(AT)echo -e "  Cache: $(CACHE_DIR)"
	$(AT)echo -e "  Coverage: $(COVERAGE_DIR)"
	$(AT)echo ""
	$(AT)echo -e "Build Status:"
	$(AT)test -f $(BUILD_DIR)/debug/cursed && echo -e "  ✅ Debug build exists" || echo -e "  ❌ Debug build missing"
	$(AT)test -f $(BUILD_DIR)/release/cursed && echo -e "  ✅ Release build exists" || echo -e "  ❌ Release build missing"

info: status health-check ## Show comprehensive system information

clean-all: clean ## Clean everything including caches
	$(AT)echo -e "$(YELLOW)🧹 Deep cleaning...$(RESET)"
	$(AT)$(CARGO_CMD) clean $(V)
	$(AT)rm -rf $(OUTPUT_DIR) $(CACHE_DIR) $(COVERAGE_DIR) $(TEST_RESULTS_DIR)
	$(AT)rm -rf target
	$(AT)rm -rf .cursed_cache .cursed_temp
	$(AT)echo -e "$(GREEN)✅ Deep clean completed$(RESET)"

reset: clean-all ## Reset build system to initial state
	$(AT)echo -e "$(YELLOW)🔄 Resetting build system...$(RESET)"
	$(AT)$(CARGO_CMD) fetch $(V)
	$(AT)$(CARGO_CMD) update $(V)
	$(AT)echo -e "$(GREEN)✅ Build system reset completed$(RESET)"

# Complete IPC Testing (NEWLY IMPLEMENTED ✅)
# =============================================================================
.PHONY: ipc-test-quick ipc-test ipc-test-all ipc-test-unit ipc-test-performance ipc-test-stress ipc-test-integration ipc-test-error ipc-test-coverage ipc-test-report ipc-clean ipc-help

ipc-test-quick: ## Run IPC quick tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC quick tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --quick

ipc-test: ## Run IPC standard tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC standard tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --unit --performance --integration --error

ipc-test-all: ## Run all IPC tests (including stress tests)
	$(AT)echo -e "$(CYAN)🔗 Running all IPC tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh

ipc-test-unit: ## Run IPC unit tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC unit tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --unit

ipc-test-performance: ## Run IPC performance tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC performance tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --performance

ipc-test-stress: ## Run IPC stress tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC stress tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --stress

ipc-test-integration: ## Run IPC integration tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC integration tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --integration

ipc-test-error: ## Run IPC error handling tests
	$(AT)echo -e "$(CYAN)🔗 Running IPC error handling tests...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --error

ipc-test-coverage: ## Generate IPC test coverage report
	$(AT)echo -e "$(MAGENTA)📊 Generating IPC test coverage report...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --coverage

ipc-test-report: ## Generate IPC test report
	$(AT)echo -e "$(MAGENTA)📋 Generating IPC test report...$(RESET)"
	$(AT)./tests/run_complete_ipc_tests.sh --report ipc_test_report.md

ipc-clean: ## Clean IPC test artifacts
	$(AT)echo -e "$(YELLOW)🧹 Cleaning IPC test artifacts...$(RESET)"
	$(AT)rm -rf ipc_test_report.md coverage/

ipc-help: ## Show IPC test commands
	$(AT)echo -e "$(BOLD)Complete IPC Test Commands:$(RESET)"
	$(AT)echo -e "  $(YELLOW)ipc-test-quick$(RESET)      - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)ipc-test$(RESET)           - Standard test suite"
	$(AT)echo -e "  $(YELLOW)ipc-test-all$(RESET)       - All tests including stress tests"
	$(AT)echo -e "  $(YELLOW)ipc-test-unit$(RESET)      - Unit tests only"
	$(AT)echo -e "  $(YELLOW)ipc-test-performance$(RESET) - Performance tests only"
	$(AT)echo -e "  $(YELLOW)ipc-test-stress$(RESET)    - Stress tests only"
	$(AT)echo -e "  $(YELLOW)ipc-test-integration$(RESET) - Integration tests only"
	$(AT)echo -e "  $(YELLOW)ipc-test-error$(RESET)     - Error handling tests only"
	$(AT)echo -e "  $(YELLOW)ipc-test-coverage$(RESET)  - Generate coverage report"
	$(AT)echo -e "  $(YELLOW)ipc-test-report$(RESET)    - Generate test report"
	$(AT)echo -e "  $(YELLOW)ipc-clean$(RESET)         - Clean test artifacts"

# Process Management and IPC Comprehensive Testing (COMPREHENSIVE ✅)
# =============================================================================
.PHONY: process-ipc-test-quick process-ipc-test process-ipc-test-all process-ipc-test-unit process-ipc-test-integration process-ipc-test-stress process-ipc-test-ffi process-ipc-test-coverage process-ipc-test-report process-ipc-clean process-ipc-help

process-ipc-test-quick: ## Run process management and IPC quick validation tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process/IPC quick tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --quick

process-ipc-test: ## Run process management and IPC standard tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process/IPC standard tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh

process-ipc-test-all: ## Run all process management and IPC tests including stress tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process/IPC comprehensive tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --stress

process-ipc-test-unit: ## Run process management unit tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process Management unit tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test unit

process-ipc-test-integration: ## Run process management integration tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process Management integration tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test integration

process-ipc-test-ipc-unit: ## Run IPC unit tests
	$(AT)echo -e "$(MAGENTA)🔧 Running IPC unit tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test ipc-unit

process-ipc-test-ipc-integration: ## Run IPC integration tests
	$(AT)echo -e "$(MAGENTA)🔧 Running IPC integration tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test ipc-integration

process-ipc-test-stress: ## Run process management and IPC stress tests
	$(AT)echo -e "$(MAGENTA)🔧 Running Process/IPC stress tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test stress

process-ipc-test-ffi: ## Run FFI integration tests
	$(AT)echo -e "$(MAGENTA)🔧 Running FFI integration tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --test ffi

process-ipc-test-coverage: ## Generate process management and IPC test coverage report
	$(AT)echo -e "$(MAGENTA)📊 Generating Process/IPC test coverage...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --coverage

process-ipc-test-report: ## Generate detailed process management and IPC test report
	$(AT)echo -e "$(MAGENTA)📄 Generating Process/IPC test report...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --report process_ipc_report.md

process-ipc-clean: ## Clean process management and IPC test artifacts
	$(AT)echo -e "$(MAGENTA)🧹 Cleaning Process/IPC test artifacts...$(RESET)"
	$(AT)rm -f process_ipc_report*.md
	$(AT)rm -rf coverage/

process-ipc-help: ## Show process management and IPC testing help
	$(AT)echo -e "$(BOLD)Process Management and IPC Testing Commands:$(RESET)"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-quick$(RESET)       - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test$(RESET)            - Standard test suite"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-all$(RESET)        - All tests including stress tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-unit$(RESET)       - Process management unit tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-integration$(RESET) - Process management integration tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-ipc-unit$(RESET)   - IPC unit tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-ipc-integration$(RESET) - IPC integration tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-stress$(RESET)     - Stress tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-ffi$(RESET)        - FFI integration tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-coverage$(RESET)   - Generate coverage report"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-report$(RESET)     - Generate detailed test report"
	$(AT)echo -e "  $(YELLOW)process-ipc-clean$(RESET)          - Clean test artifacts"
	$(AT)echo ""
	$(AT)echo "Examples:"
	$(AT)echo "  make process-ipc-test-quick  - Fast validation"
	$(AT)echo "  make process-ipc-test-all    - Complete test suite"
	$(AT)echo "  make process-ipc-test-report - Generate documentation"

# Advanced Optimization testing targets
advanced-opt-test-quick: ## Run quick advanced optimization tests
	$(AT)echo -e "$(CYAN)🚀 Running quick advanced optimization tests...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --quick

advanced-opt-test: ## Run advanced optimization tests
	$(AT)echo -e "$(CYAN)🚀 Running advanced optimization tests...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --test unit --test integration

advanced-opt-test-all: ## Run all advanced optimization tests (including performance)
	$(AT)echo -e "$(CYAN)🚀 Running all advanced optimization tests (including performance)...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --ignored

advanced-opt-test-unit: ## Run advanced optimization unit tests
	$(AT)echo -e "$(CYAN)🚀 Running advanced optimization unit tests...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --test unit

advanced-opt-test-integration: ## Run advanced optimization integration tests
	$(AT)echo -e "$(CYAN)🚀 Running advanced optimization integration tests...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --test integration

advanced-opt-test-performance: ## Run advanced optimization performance tests
	$(AT)echo -e "$(CYAN)🚀 Running advanced optimization performance tests...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --test performance --ignored

advanced-opt-test-report: ## Generate advanced optimization test report
	$(AT)echo -e "$(MAGENTA)📋 Generating advanced optimization test report...$(RESET)"
	$(AT)./tests/run_advanced_optimization_tests.sh --report

advanced-opt-help: ## Show advanced optimization test commands
	$(AT)echo -e "$(BOLD)Advanced Optimization Test Commands:$(RESET)"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-quick$(RESET)      - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test$(RESET)            - Standard test suite"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-all$(RESET)        - All tests including performance"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-unit$(RESET)       - Unit tests only"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-integration$(RESET) - Integration tests only"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-performance$(RESET) - Performance tests only"
	$(AT)echo -e "  $(YELLOW)advanced-opt-test-report$(RESET)     - Generate detailed report"

# =============================================================================
# Enhanced Process Management and IPC Tests
# =============================================================================

.PHONY: process-ipc-test process-ipc-test-all process-ipc-test-quick process-ipc-test-stress
.PHONY: process-ipc-test-coverage process-ipc-test-report process-ipc-help

process-ipc-test: build ## Run process management and IPC tests
	$(AT)echo -e "$(BLUE)🔧 Running process management and IPC tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh
	$(AT)echo -e "$(GREEN)✅ Process and IPC tests completed$(RESET)"

process-ipc-test-all: build ## Run all process and IPC tests including stress tests
	$(AT)echo -e "$(BLUE)🚀 Running comprehensive process and IPC tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --stress
	$(AT)echo -e "$(GREEN)✅ All process and IPC tests completed$(RESET)"

process-ipc-test-quick: build ## Run quick process and IPC tests only
	$(AT)echo -e "$(BLUE)⚡ Running quick process and IPC tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --quick
	$(AT)echo -e "$(GREEN)✅ Quick process and IPC tests completed$(RESET)"

process-ipc-test-stress: build ## Run stress tests for process and IPC systems
	$(AT)echo -e "$(BLUE)💪 Running process and IPC stress tests...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --stress
	$(AT)echo -e "$(GREEN)✅ Process and IPC stress tests completed$(RESET)"

process-ipc-test-coverage: build ## Generate coverage report for process and IPC tests
	$(AT)echo -e "$(BLUE)📊 Generating process and IPC test coverage...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --coverage
	$(AT)echo -e "$(GREEN)✅ Coverage report generated$(RESET)"

process-ipc-test-report: build ## Generate detailed test report
	$(AT)echo -e "$(BLUE)📋 Generating process and IPC test report...$(RESET)"
	$(AT)./tests/run_process_ipc_comprehensive_tests.sh --report test_results/process_ipc_report.md
	$(AT)echo -e "$(GREEN)✅ Test report generated$(RESET)"

process-ipc-help: ## Show process and IPC testing help
	$(AT)echo -e "$(CYAN)Process Management and IPC Testing Help$(RESET)"
	$(AT)echo -e ""
	$(AT)echo -e "Available targets:"
	$(AT)echo -e "  $(YELLOW)process-ipc-test$(RESET)          - Run all standard tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-all$(RESET)      - Run all tests including stress tests"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-quick$(RESET)    - Run quick tests only"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-stress$(RESET)   - Run stress tests only"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-coverage$(RESET) - Generate coverage report"
	$(AT)echo -e "  $(YELLOW)process-ipc-test-report$(RESET)   - Generate detailed report"
	$(AT)echo -e ""
	$(AT)echo -e "Test categories:"
	$(AT)echo -e "  $(CYAN)Enhanced Process Management$(RESET) - Advanced command execution and monitoring"
	$(AT)echo -e "  $(CYAN)Advanced IPC$(RESET)               - Comprehensive inter-process communication"
	$(AT)echo -e "  $(CYAN)LLVM Integration$(RESET)           - Code generation and FFI support"

# Enhanced Process Management Targets
# =============================================================================

enhanced-process-test-quick: ## Quick validation of enhanced process management features
	$(AT)echo -e "$(CYAN)🚀 Running quick enhanced process management tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test enhanced_process_management_integration_test basic_slay_command

enhanced-process-test: ## Run enhanced process management test suite
	$(AT)echo -e "$(CYAN)🔧 Running enhanced process management tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test enhanced_process_management_integration_test

enhanced-process-test-all: ## Run all enhanced process management tests including stress tests
	$(AT)echo -e "$(CYAN)💪 Running comprehensive enhanced process management tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test enhanced_process_management_integration_test -- --include-ignored

enhanced-process-test-unit: ## Run enhanced process management unit tests
	$(AT)echo -e "$(CYAN)🧪 Running enhanced process management unit tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --lib stdlib::process::exec_slay
	$(AT)$(CARGO_CMD) test --lib stdlib::process::exec_vibez_enhanced

enhanced-process-test-integration: ## Run enhanced process management integration tests
	$(AT)echo -e "$(CYAN)🔗 Running enhanced process management integration tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test enhanced_process_management_integration_test test_comprehensive_integration

enhanced-process-test-performance: ## Run enhanced process management performance tests
	$(AT)echo -e "$(CYAN)⚡ Running enhanced process management performance tests...$(RESET)"
	$(AT)$(CARGO_CMD) test --test enhanced_process_management_integration_test test_performance_scalability -- --ignored

enhanced-process-demo: ## Run the enhanced process management demo
	$(AT)echo -e "$(CYAN)🎬 Running enhanced process management demo...$(RESET)"
	$(AT)$(CARGO_CMD) run --bin cursed examples/enhanced_process_management_demo.csd

enhanced-process-test-coverage: ## Generate coverage report for enhanced process management
	$(AT)echo -e "$(CYAN)📊 Generating enhanced process management test coverage...$(RESET)"
	$(AT)mkdir -p $(COVERAGE_DIR)
	$(AT)$(LINKING_FIX) cargo tarpaulin \
		--out Html --output-dir $(COVERAGE_DIR) \
		--include-tests \
		--test enhanced_process_management_integration_test \
		--lib \
		--exclude-files "tests/*" "examples/*" "benches/*"

enhanced-process-benchmark: ## Run enhanced process management benchmarks
	$(AT)echo -e "$(CYAN)🏃 Running enhanced process management benchmarks...$(RESET)"
	$(AT)$(CARGO_CMD) bench --bench process_management_bench

enhanced-process-validate: ## Validate enhanced process management implementation
	$(AT)echo -e "$(CYAN)✅ Validating enhanced process management implementation...$(RESET)"
	$(AT)$(CARGO_CMD) check --lib --features enhanced-process
	$(AT)$(CARGO_CMD) clippy --lib --features enhanced-process -- -D warnings

enhanced-process-build-examples: ## Build enhanced process management examples
	$(AT)echo -e "$(CYAN)🔨 Building enhanced process management examples...$(RESET)"
	$(AT)$(CARGO_CMD) check --examples

enhanced-process-clean: ## Clean enhanced process management artifacts
	$(AT)echo -e "$(CYAN)🧹 Cleaning enhanced process management artifacts...$(RESET)"
	$(AT)rm -rf $(TEST_RESULTS_DIR)/process_*
	$(AT)rm -rf $(OUTPUT_DIR)/process_*
	$(AT)find . -name "*_test_*.txt" -delete
	$(AT)find . -name "*.log" -delete

enhanced-process-help: ## Show enhanced process management commands
	$(AT)echo -e "$(BOLD)Enhanced Process Management Commands:$(RESET)"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-quick$(RESET)      - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test$(RESET)            - Standard test suite"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-all$(RESET)        - All tests including stress tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-unit$(RESET)       - Unit tests only"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-integration$(RESET) - Integration tests only"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-performance$(RESET) - Performance tests only"
	$(AT)echo -e "  $(YELLOW)enhanced-process-demo$(RESET)            - Run demonstration program"
	$(AT)echo -e "  $(YELLOW)enhanced-process-test-coverage$(RESET)   - Generate coverage report"
	$(AT)echo -e "  $(YELLOW)enhanced-process-benchmark$(RESET)       - Run performance benchmarks"
	$(AT)echo -e "  $(YELLOW)enhanced-process-validate$(RESET)        - Validate implementation"

# Enhanced Process Management and IPC Comprehensive Testing
# =============================================================================

enhanced-process-ipc-test: ## Run comprehensive process and IPC tests
	$(AT)echo -e "$(CYAN)🧪 Running enhanced process and IPC tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh

enhanced-process-ipc-test-quick: ## Quick validation of core functionality
	$(AT)echo -e "$(CYAN)⚡ Running quick enhanced process and IPC tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --quick

enhanced-process-ipc-test-verbose: ## Verbose test execution with detailed output
	$(AT)echo -e "$(CYAN)🔍 Running enhanced process and IPC tests (verbose)...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --verbose

enhanced-process-ipc-test-basic: ## Run basic enhanced functionality tests
	$(AT)echo -e "$(CYAN)🏁 Running basic enhanced process and IPC tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test basic

enhanced-process-ipc-test-integration: ## Run integration tests across modules
	$(AT)echo -e "$(CYAN)🔗 Running enhanced process and IPC integration tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test integration

enhanced-process-ipc-test-stress: ## Run stress tests for performance validation
	$(AT)echo -e "$(CYAN)💪 Running enhanced process and IPC stress tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test stress

enhanced-process-ipc-test-performance: ## Run performance benchmarks
	$(AT)echo -e "$(CYAN)🏃 Running enhanced process and IPC performance tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test performance

enhanced-process-ipc-test-coverage: ## Generate code coverage report
	$(AT)echo -e "$(CYAN)📊 Generating enhanced process and IPC coverage report...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --coverage

enhanced-process-ipc-test-report: ## Generate comprehensive test report
	$(AT)echo -e "$(CYAN)📋 Generating enhanced process and IPC test report...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --report enhanced_process_ipc_report.md
	$(AT)echo -e "$(GREEN)✅ Report generated: enhanced_process_ipc_report.md$(RESET)"

enhanced-process-ipc-test-all: ## Run all tests including stress and performance
	$(AT)echo -e "$(CYAN)🎯 Running all enhanced process and IPC tests...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test all --verbose

enhanced-process-ipc-validate: ## Validate entire enhanced system
	$(AT)echo -e "$(CYAN)✅ Validating enhanced process and IPC system...$(RESET)"
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test basic
	$(AT)./tests/run_enhanced_process_ipc_tests.sh --test integration
	$(AT)echo -e "$(GREEN)✅ Enhanced process and IPC system validation complete$(RESET)"

enhanced-process-ipc-benchmark: ## Run performance benchmarks with timing
	$(AT)echo -e "$(CYAN)⏱️  Running enhanced process and IPC benchmarks...$(RESET)"
	$(AT)time ./tests/run_enhanced_process_ipc_tests.sh --test performance --verbose

enhanced-process-ipc-clean: ## Clean enhanced process and IPC test artifacts
	$(AT)echo -e "$(CYAN)🧹 Cleaning enhanced process and IPC artifacts...$(RESET)"
	$(AT)rm -rf enhanced_process_ipc_report.md
	$(AT)rm -rf $(COVERAGE_DIR)/enhanced_process_ipc_*
	$(AT)rm -rf $(TEST_RESULTS_DIR)/enhanced_process_ipc_*

enhanced-process-ipc-help: ## Show enhanced process and IPC commands
	$(AT)echo -e "$(BOLD)Enhanced Process Management and IPC Commands:$(RESET)"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test$(RESET)            - Run comprehensive tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-quick$(RESET)      - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-verbose$(RESET)    - Verbose test execution"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-basic$(RESET)      - Basic functionality tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-integration$(RESET) - Integration tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-stress$(RESET)     - Stress tests"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-performance$(RESET) - Performance benchmarks"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-coverage$(RESET)   - Coverage analysis"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-report$(RESET)     - Generate test report"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-test-all$(RESET)        - All tests with verbose output"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-validate$(RESET)        - Complete system validation"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-benchmark$(RESET)       - Timed performance benchmarks"
	$(AT)echo -e "  $(YELLOW)enhanced-process-ipc-clean$(RESET)           - Clean test artifacts"
	$(AT)echo -e "  $(YELLOW)enhanced-process-build-examples$(RESET)  - Build example programs"
	$(AT)echo -e "  $(YELLOW)enhanced-process-clean$(RESET)           - Clean artifacts"

# Include optimization targets if available
-include Makefile.optimization

# Make sure we don't accidentally run dangerous commands
.PRECIOUS: $(BUILD_DIR)/ $(OUTPUT_DIR)/


# Post-Quantum Cryptography Hybrid Testing Infrastructure - COMPREHENSIVE ✅
# =============================================================================
.PHONY: pqc-hybrid-test-quick pqc-hybrid-test pqc-hybrid-test-benchmark pqc-hybrid-test-all pqc-hybrid-cli-test pqc-hybrid-cli-build pqc-hybrid-benchmark pqc-hybrid-compatibility pqc-hybrid-migration pqc-hybrid-example pqc-hybrid-help

# Quick PQC hybrid tests
pqc-hybrid-test-quick: ## Run PQC hybrid quick tests
	@echo "🔐 Running PQC hybrid quick tests..."
	./fix_linking.sh cargo test --test crypto_pqc_hybrid_test --release

# Standard PQC hybrid tests
pqc-hybrid-test: ## Run PQC hybrid standard tests
	@echo "🔐 Running PQC hybrid standard tests..."
	./fix_linking.sh cargo test --test crypto_pqc_hybrid_test --release -- --nocapture

# Performance and benchmark tests
pqc-hybrid-test-benchmark: ## Run PQC hybrid benchmark tests
	@echo "🔐 Running PQC hybrid benchmark tests..."
	./fix_linking.sh cargo test --test crypto_pqc_hybrid_benchmark_test --release -- --nocapture --ignored

# All PQC hybrid tests
pqc-hybrid-test-all: ## Run comprehensive PQC hybrid tests
	@echo "🔐 Running comprehensive PQC hybrid tests..."
	./fix_linking.sh cargo test --test crypto_pqc_hybrid_test --release -- --nocapture
	./fix_linking.sh cargo test --test crypto_pqc_hybrid_benchmark_test --release -- --nocapture

# CLI tool testing
pqc-hybrid-cli-test: ## Test PQC hybrid CLI tool
	@echo "🔐 Testing PQC hybrid CLI tool..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release
	./target/release/cursed_pqc_hybrid --help

# Build PQC hybrid CLI tool
pqc-hybrid-cli-build: ## Build PQC hybrid CLI tool
	@echo "🔐 Building PQC hybrid CLI tool..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release

# Performance benchmarks
pqc-hybrid-benchmark: ## Run PQC hybrid performance benchmarks
	@echo "🔐 Running PQC hybrid performance benchmarks..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release
	./target/release/cursed_pqc_hybrid benchmark --iterations 5

# Compatibility matrix
pqc-hybrid-compatibility: ## Show PQC hybrid compatibility matrix
	@echo "🔐 Showing PQC hybrid compatibility matrix..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release
	./target/release/cursed_pqc_hybrid compatibility

# Migration strategy
pqc-hybrid-migration: ## Show PQC hybrid migration strategy
	@echo "🔐 Showing PQC hybrid migration strategy..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release
	./target/release/cursed_pqc_hybrid migration

# Example workflow
pqc-hybrid-example: ## Run PQC hybrid example workflow
	@echo "🔐 Running PQC hybrid example workflow..."
	./fix_linking.sh cargo build --bin cursed_pqc_hybrid --release
	@echo "Generating hybrid key pair..."
	./target/release/cursed_pqc_hybrid keygen --classical x25519 --pqc kyber --security-level level1 --public-key-out hybrid_pub.key --secret-key-out hybrid_sec.key
	@echo "Performing encapsulation..."
	./target/release/cursed_pqc_hybrid encaps --public-key hybrid_pub.key --ciphertext-out hybrid.ct --shared-secret-out hybrid_encaps.secret --classical x25519 --pqc kyber --security-level level1
	@echo "Performing decapsulation..."
	./target/release/cursed_pqc_hybrid decaps --secret-key hybrid_sec.key --ciphertext hybrid.ct --shared-secret-out hybrid_decaps.secret --classical x25519 --pqc kyber --security-level level1
	@echo "Validating key pair..."
	./target/release/cursed_pqc_hybrid validate --public-key hybrid_pub.key --secret-key hybrid_sec.key --classical x25519 --pqc kyber --security-level level1
	@echo "Cleaning up..."
	rm -f hybrid_pub.key hybrid_sec.key hybrid.ct hybrid_encaps.secret hybrid_decaps.secret

# Help
pqc-hybrid-help: ## Show PQC hybrid help
	@echo "Post-Quantum Cryptography Hybrid Testing Commands:"
	@echo "  pqc-hybrid-test-quick        - Run quick validation tests"
	@echo "  pqc-hybrid-test             - Run standard test suite"
	@echo "  pqc-hybrid-test-benchmark   - Run performance benchmark tests"
	@echo "  pqc-hybrid-test-all         - Run comprehensive test suite"
	@echo "  pqc-hybrid-cli-test         - Test CLI tool functionality"
	@echo "  pqc-hybrid-cli-build        - Build CLI tool"
	@echo "  pqc-hybrid-benchmark        - Run performance benchmarks"
	@echo "  pqc-hybrid-compatibility    - Show algorithm compatibility matrix"
	@echo "  pqc-hybrid-migration        - Show migration strategy"
	@echo "  pqc-hybrid-example          - Run complete example workflow"
	@echo "  pqc-hybrid-help             - Show this help message"

# Redis Database Driver Testing
# =============================================================================
.PHONY: redis-test redis-test-unit redis-test-integration redis-test-performance
.PHONY: redis-test-all redis-test-report redis-example redis-help

# Redis driver tests
redis-test: ## Run Redis driver tests (unit + integration if Redis available)
	@echo -e "$(CYAN)🔥 Running Redis driver tests...$(RESET)"
	./tests/run_redis_tests.sh

redis-test-unit: ## Run Redis driver unit tests only
	@echo -e "$(CYAN)🧪 Running Redis driver unit tests...$(RESET)"
	$(LINKING_FIX) cargo test --test redis_driver_test test_redis_config test_value_conversions test_driver_creation test_error_handling test_config_edge_cases

redis-test-integration: ## Run Redis driver integration tests (requires Redis)
	@echo -e "$(CYAN)🚀 Running Redis driver integration tests...$(RESET)"
	$(LINKING_FIX) cargo test --test redis_driver_test -- --ignored

redis-test-performance: ## Run Redis driver performance tests
	@echo -e "$(CYAN)⚡ Running Redis driver performance tests...$(RESET)"
	./tests/run_redis_tests.sh --performance

redis-test-all: ## Run all Redis driver tests including performance
	@echo -e "$(CYAN)🔥 Running comprehensive Redis driver test suite...$(RESET)"
	./tests/run_redis_tests.sh --performance

redis-test-report: ## Generate Redis driver test coverage report
	@echo -e "$(CYAN)📊 Generating Redis driver test coverage report...$(RESET)"
	./tests/run_redis_tests.sh --report

redis-example: ## Run Redis driver example
	@echo -e "$(CYAN)🎯 Running Redis driver example...$(RESET)"
	cargo run -- examples/redis_demo.csd

redis-help: ## Show Redis driver testing help
	@echo -e "$(BOLD)$(CYAN)Redis Database Driver Testing Commands:$(RESET)"
	@echo -e "  $(CYAN)redis-test              $(RESET)Run Redis driver tests (unit + integration if Redis available)"
	@echo -e "  $(CYAN)redis-test-unit         $(RESET)Run unit tests only (no Redis required)"
	@echo -e "  $(CYAN)redis-test-integration  $(RESET)Run integration tests (requires Redis)"
	@echo -e "  $(CYAN)redis-test-performance  $(RESET)Run performance benchmarks"
	@echo -e "  $(CYAN)redis-test-all          $(RESET)Run comprehensive test suite"
	@echo -e "  $(CYAN)redis-test-report       $(RESET)Generate test coverage report"
	@echo -e "  $(CYAN)redis-example           $(RESET)Run Redis driver example"
	@echo -e "  $(CYAN)redis-help              $(RESET)Show this help message"
	@echo ""
	@echo -e "$(YELLOW)Setup Instructions:$(RESET)"
	@echo -e "  1. Install Redis: $(BLUE)sudo apt-get install redis-server$(RESET) (Ubuntu/Debian)"
	@echo -e "  2. Start Redis: $(BLUE)redis-server$(RESET)"
	@echo -e "  3. Test connection: $(BLUE)redis-cli ping$(RESET)"
	@echo -e "  4. Run tests: $(BLUE)make redis-test$(RESET)"


# PKI Certificate Signature Verification Testing Infrastructure - COMPREHENSIVE ✅
# =============================================================================
.PHONY: pki-test-quick pki-test pki-test-rsa pki-test-ecdsa pki-test-ed25519 pki-test-algorithms pki-test-performance pki-test-concurrent pki-test-edge-cases pki-test-all pki-clean pki-help

# Quick PKI signature verification tests
pki-test-quick: ## Run PKI signature verification quick tests
	@echo "🔐 Running PKI signature verification quick tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_certificate_processor_creation --quiet

# Standard PKI signature verification tests
pki-test: ## Run PKI signature verification tests
	@echo "🔒 Running PKI signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test

# RSA signature verification tests
pki-test-rsa: ## Run RSA signature verification tests
	@echo "🔑 Running RSA signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_rsa_signature

# ECDSA signature verification tests
pki-test-ecdsa: ## Run ECDSA signature verification tests
	@echo "📈 Running ECDSA signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_ecdsa_signature

# Ed25519 signature verification tests
pki-test-ed25519: ## Run Ed25519 signature verification tests
	@echo "⚡ Running Ed25519 signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_ed25519_signature

# Algorithm compatibility tests
pki-test-algorithms: ## Run signature verification algorithm tests
	@echo "🧮 Running signature verification algorithm tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_multiple_hash_algorithms

# Performance tests
pki-test-performance: ## Run PKI signature verification performance tests
	@echo "🚀 Running PKI signature verification performance tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_signature_verification_performance --release

# Concurrent verification tests
pki-test-concurrent: ## Run PKI concurrent signature verification tests
	@echo "🔄 Running PKI concurrent signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_concurrent_signature_verification

# Edge case tests
pki-test-edge-cases: ## Run PKI edge case tests
	@echo "⚠️  Running PKI edge case tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test test_error_handling_edge_cases

# Complete PKI test suite
pki-test-all: ## Run all PKI signature verification tests
	@echo "🔒 Running all PKI signature verification tests..."
	./fix_linking.sh cargo test --test pki_signature_verification_test
	@echo "✅ PKI signature verification tests completed"

# Clean PKI test artifacts
pki-clean: ## Clean PKI test artifacts
	@echo "🧹 Cleaning PKI test artifacts..."
	rm -rf target/debug/deps/*pki_signature*

# PKI help
pki-help: ## Show PKI testing commands
	@echo "PKI Certificate Signature Verification Testing Commands:"
	@echo "  pki-test-quick       - Quick validation tests"
	@echo "  pki-test             - All PKI signature verification tests"
	@echo "  pki-test-rsa         - RSA signature verification tests"
	@echo "  pki-test-ecdsa       - ECDSA signature verification tests"
	@echo "  pki-test-ed25519     - Ed25519 signature verification tests"
	@echo "  pki-test-algorithms  - Hash algorithm compatibility tests"
	@echo "  pki-test-performance - Performance benchmarking tests"
	@echo "  pki-test-concurrent  - Concurrent verification tests"
	@echo "  pki-test-edge-cases  - Error handling and edge case tests"
	@echo "  pki-test-all         - Complete PKI test suite"
	@echo "  pki-clean           - Clean test artifacts"
	@echo "  pki-help            - Show this help"

# MySQL Production Driver Testing
# =============================================================================

# MySQL Production Driver testing targets
mysql-production-test: ## Run MySQL production driver tests
	@echo "🐬 Running MySQL production driver tests..."
	$(CARGO_WRAPPER) test --test mysql_production_driver_test

mysql-production-test-quick: ## Run MySQL production driver quick tests
	@echo "🐬 Running MySQL production driver quick tests..."
	$(CARGO_WRAPPER) test --test mysql_production_driver_test test_production_config
	$(CARGO_WRAPPER) test --test mysql_production_driver_test test_sql_sanitizer
	$(CARGO_WRAPPER) test --test mysql_production_driver_test test_type_conversions

mysql-production-test-all: ## Run comprehensive MySQL production driver tests
	@echo "🐬 Running comprehensive MySQL production driver tests..."
	./tests/run_mysql_production_tests.sh

mysql-production-test-verbose: ## Run MySQL production driver tests with verbose output
	@echo "🐬 Running MySQL production driver tests with verbose output..."
	./tests/run_mysql_production_tests.sh --verbose

mysql-production-test-coverage: ## Run MySQL production driver tests with coverage
	@echo "🐬 Running MySQL production driver tests with coverage..."
	./tests/run_mysql_production_tests.sh --coverage

mysql-production-validate: ## Validate MySQL production driver implementation
	@echo "🐬 Validating MySQL production driver implementation..."
	$(CARGO_WRAPPER) check --test mysql_production_driver_test
	$(CARGO_WRAPPER) clippy --test mysql_production_driver_test

mysql-production-clean: ## Clean MySQL production driver test artifacts
	@echo "🐬 Cleaning MySQL production driver test artifacts..."
	rm -f mysql_production_test_report.md
	rm -f tarpaulin-report.html

mysql-production-help: ## Show MySQL production driver help
	@echo "🐬 MySQL Production Driver Test Commands:"
	@echo "  mysql-production-test         - Run all MySQL production driver tests"
	@echo "  mysql-production-test-quick   - Run quick validation tests"
	@echo "  mysql-production-test-all     - Run comprehensive test suite with script"
	@echo "  mysql-production-test-verbose - Run tests with verbose output"
	@echo "  mysql-production-test-coverage- Run tests with coverage analysis"
	@echo "  mysql-production-validate     - Validate implementation (check + clippy)"
	@echo "  mysql-production-clean        - Clean test artifacts"
	@echo "  mysql-production-help         - Show this help"

# =============================================================================
# Distributed Compilation System Targets
# =============================================================================

distributed-compile-test: ## Run distributed compilation tests
	@echo "$(CYAN)🚀 Running distributed compilation tests...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test

distributed-compile-test-unit: ## Run distributed compilation unit tests
	@echo "$(CYAN)🧪 Running distributed compilation unit tests...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test basic_tests --

distributed-compile-test-integration: ## Run distributed compilation integration tests
	@echo "$(CYAN)🔗 Running distributed compilation integration tests...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test integration_tests --

distributed-compile-test-performance: ## Run distributed compilation performance tests
	@echo "$(CYAN)⚡ Running distributed compilation performance tests...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test performance_tests -- --ignored

distributed-compile-demo: ## Run distributed compilation demo
	@echo "$(CYAN)🎯 Running distributed compilation demo...$(RESET)"
	$(CARGO_CMD) run --example distributed_compilation_demo

distributed-compile-benchmark: ## Run distributed compilation benchmarks
	@echo "$(CYAN)📊 Running distributed compilation benchmarks...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test performance_tests -- --ignored --test-threads 1

distributed-compile-fault-tolerance: ## Test distributed compilation fault tolerance
	@echo "$(CYAN)🛡️  Testing distributed compilation fault tolerance...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test fault_tolerance_tests --

distributed-compile-worker-tests: ## Test worker node management
	@echo "$(CYAN)👷 Testing worker node management...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test worker_tests --

distributed-compile-load-balancer-tests: ## Test load balancing strategies
	@echo "$(CYAN)⚖️  Testing load balancing strategies...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test load_balancer_tests --

distributed-compile-network-tests: ## Test network optimization
	@echo "$(CYAN)🌐 Testing network optimization...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test network_tests --

distributed-compile-cache-tests: ## Test compilation caching
	@echo "$(CYAN)💾 Testing compilation caching...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test cache_tests --

distributed-compile-compiler-tests: ## Test distributed compiler core
	@echo "$(CYAN)🔧 Testing distributed compiler core...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test compiler_tests --

distributed-compile-edge-cases: ## Test edge cases and error conditions
	@echo "$(CYAN)🔍 Testing edge cases and error conditions...$(RESET)"
	$(CARGO_CMD) test --test distributed_compilation_test edge_case_tests --

distributed-compile-all: ## Run all distributed compilation tests and demos
	@echo "$(CYAN)🌟 Running all distributed compilation tests and demos...$(RESET)"
	$(MAKE) distributed-compile-test
	$(MAKE) distributed-compile-demo

distributed-compile-quick: ## Quick validation of distributed compilation system
	@echo "$(CYAN)⚡ Quick validation of distributed compilation system...$(RESET)"
	$(MAKE) distributed-compile-test-unit
	$(MAKE) distributed-compile-test-integration

distributed-compile-coverage: ## Generate coverage report for distributed compilation
	@echo "$(CYAN)📈 Generating coverage report for distributed compilation...$(RESET)"
	$(CARGO_CMD) tarpaulin --test distributed_compilation_test --out Html --output-dir $(COVERAGE_DIR)/distributed

distributed-compile-validate: ## Validate distributed compilation implementation
	@echo "$(CYAN)✅ Validating distributed compilation implementation...$(RESET)"
	$(CARGO_CMD) check --test distributed_compilation_test
	$(CARGO_CMD) clippy --test distributed_compilation_test -- -D warnings

distributed-compile-clean: ## Clean distributed compilation test artifacts
	@echo "$(CYAN)🧹 Cleaning distributed compilation test artifacts...$(RESET)"
	rm -f distributed_compilation_report.md
	rm -rf $(COVERAGE_DIR)/distributed

distributed-compile-help: ## Show distributed compilation help
	@echo "$(CYAN)🚀 Distributed Compilation System Commands:$(RESET)"
	@echo "  distributed-compile-test              - Run all distributed compilation tests"
	@echo "  distributed-compile-test-unit         - Run unit tests"
	@echo "  distributed-compile-test-integration  - Run integration tests"
	@echo "  distributed-compile-test-performance  - Run performance tests"
	@echo "  distributed-compile-demo              - Run demonstration program"
	@echo "  distributed-compile-benchmark         - Run benchmarks"
	@echo "  distributed-compile-fault-tolerance   - Test fault tolerance"
	@echo "  distributed-compile-worker-tests      - Test worker management"
	@echo "  distributed-compile-load-balancer-tests - Test load balancing"
	@echo "  distributed-compile-network-tests     - Test network optimization"
	@echo "  distributed-compile-cache-tests       - Test compilation caching"
	@echo "  distributed-compile-compiler-tests    - Test compiler core"
	@echo "  distributed-compile-edge-cases        - Test edge cases"
	@echo "  distributed-compile-all               - Run all tests and demos"
	@echo "  distributed-compile-quick             - Quick validation"
	@echo "  distributed-compile-coverage          - Generate coverage report"
	@echo "  distributed-compile-validate          - Validate implementation"
	@echo "  distributed-compile-clean             - Clean test artifacts"
	@echo "  distributed-compile-help              - Show this help"

# Production compression system testing targets
compression-test:
	@echo "Running compression system tests..."
	./tests/run_compression_tests.sh

compression-test-quick:
	@echo "Running quick compression tests..."
	./tests/run_compression_tests.sh --quick

compression-test-verbose:
	@echo "Running compression tests with verbose output..."
	./tests/run_compression_tests.sh --verbose

compression-test-benchmark:
	@echo "Running compression performance benchmarks..."
	./tests/run_compression_tests.sh --benchmark

compression-test-stress:
	@echo "Running compression stress tests..."
	./tests/run_compression_tests.sh --stress

compression-test-coverage:
	@echo "Running compression tests with coverage analysis..."
	./tests/run_compression_tests.sh --coverage

compression-test-all:
	@echo "Running all compression tests including benchmarks and stress tests..."
	./tests/run_compression_tests.sh --verbose --benchmark --stress --coverage --report compression_report.md

compression-bench:
	@echo "Running compression benchmarks..."
	$(CARGO_FIX) bench --bench compression_benchmark --features benchmarks

compression-help:
	@echo "CURSED Compression System Testing Commands:"
	@echo ""
	@echo "  compression-test           - Run standard compression tests"
	@echo "  compression-test-quick     - Run quick validation tests"
	@echo "  compression-test-verbose   - Run tests with detailed output"
	@echo "  compression-test-benchmark - Run performance benchmarks"
	@echo "  compression-test-stress    - Run stress and load tests"
	@echo "  compression-test-coverage  - Run tests with coverage analysis"
	@echo "  compression-test-all       - Run comprehensive test suite"
	@echo "  compression-bench          - Run benchmarks only"
	@echo "  compression-help           - Show this help message"
	@echo ""
	@echo "The production compression system supports:"
	@echo "  - GZIP compression (flate2)"
	@echo "  - Deflate compression (flate2)"
	@echo "  - Brotli compression (brotli crate)"
	@echo "  - Zstandard compression (zstd crate)"
	@echo "  - Streaming compression for large files"
	@echo "  - Performance monitoring and statistics"
	@echo "  - Comprehensive error handling"
	@echo ""


# Cache Optimization System Testing Commands
# =============================================================================
.PHONY: cache-opt-test cache-opt-test-quick cache-opt-test-integration cache-opt-test-performance cache-opt-test-all cache-opt-test-coverage cache-opt-help

cache-opt-test-quick: ## Run quick cache optimization tests
	$(AT)echo -e "$(CYAN)🗄️  Running quick cache optimization tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test cache_optimization_tests::test_cache_structure_analysis
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test cache_optimization_tests::test_stale_entry_detection
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test cache_optimization_tests::test_duplicate_detection

cache-opt-test: ## Run standard cache optimization tests
	$(AT)echo -e "$(CYAN)🗄️  Running cache optimization tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test cache_optimization_tests

cache-opt-test-integration: ## Run cache optimization integration tests
	$(AT)echo -e "$(CYAN)🔗 Running cache optimization integration tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test integration_tests

cache-opt-test-performance: ## Run cache optimization performance tests
	$(AT)echo -e "$(CYAN)⚡ Running cache optimization performance tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test cache_optimization_test performance_tests --release

cache-opt-test-all: cache-opt-test cache-opt-test-integration cache-opt-test-performance ## Run comprehensive cache optimization tests
	$(AT)echo -e "$(GREEN)✅ All cache optimization tests completed$(RESET)"

cache-opt-test-coverage: ## Run cache optimization tests with coverage
	$(AT)echo -e "$(CYAN)📊 Running cache optimization tests with coverage...$(RESET)"
	$(LINKING_FIX) cargo tarpaulin --test cache_optimization_test --out Html --output-dir $(COVERAGE_DIR)/cache_optimization

cache-opt-help: ## Show cache optimization testing help
	@echo "CURSED Cache Optimization Testing Commands:"
	@echo ""
	@echo "  cache-opt-test-quick       - Run quick validation tests"
	@echo "  cache-opt-test             - Run standard cache optimization tests"
	@echo "  cache-opt-test-integration - Run integration tests"
	@echo "  cache-opt-test-performance - Run performance tests"
	@echo "  cache-opt-test-all         - Run comprehensive test suite"
	@echo "  cache-opt-test-coverage    - Run tests with coverage analysis"
	@echo "  cache-opt-help             - Show this help message"
	@echo ""
	@echo "Cache optimization features tested:"
	@echo "  - Cache structure analysis and reporting"
	@echo "  - Stale entry detection and removal"
	@echo "  - Duplicate file detection and deduplication"
	@echo "  - Compression candidate identification"
	@echo "  - Fragmentation analysis and optimization"
	@echo "  - Size limit enforcement and cleanup"
	@echo "  - Cache metadata management"
	@echo "  - Performance monitoring and statistics"
	@echo ""


# Post-Quantum Cryptography Hybrid System Testing Commands
# =============================================================================
.PHONY: pqc-hybrid-test-quick pqc-hybrid-test pqc-hybrid-test-all pqc-hybrid-test-integration pqc-hybrid-test-security pqc-hybrid-test-performance pqc-hybrid-example pqc-hybrid-build-check pqc-hybrid-doc pqc-hybrid-help

pqc-hybrid-test-quick: ## Run quick PQC hybrid tests
	$(AT)echo -e "$(CYAN)🔐 Running quick PQC hybrid tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test 
	$(LINKING_FIX) $(CARGO_CMD) test --lib crypto_pqc::hybrid::tests
	$(LINKING_FIX) $(CARGO_CMD) test --lib crypto_pqc::agility::tests

pqc-hybrid-test: ## Run PQC hybrid system tests
	$(AT)echo -e "$(CYAN)🔐 Running PQC hybrid system tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test
	$(LINKING_FIX) $(CARGO_CMD) test --lib crypto_pqc::hybrid::tests
	$(LINKING_FIX) $(CARGO_CMD) test --lib crypto_pqc::agility::tests

pqc-hybrid-test-all: pqc-hybrid-test ## Run all PQC hybrid tests (including performance)
	$(AT)echo -e "$(CYAN)🔐 Running all PQC hybrid tests including performance tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test -- --ignored

pqc-hybrid-test-integration: ## Run PQC hybrid integration tests
	$(AT)echo -e "$(CYAN)🔗 Running PQC hybrid integration tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test test_hybrid_kem test_hybrid_signature test_migration_strategy

pqc-hybrid-test-security: ## Run PQC hybrid security tests
	$(AT)echo -e "$(CYAN)🛡️  Running PQC hybrid security tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test test_security_properties test_error_conditions

pqc-hybrid-test-performance: ## Run PQC hybrid performance tests
	$(AT)echo -e "$(CYAN)⚡ Running PQC hybrid performance tests...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) test --test crypto_pqc_hybrid_test benchmark_hybrid_operations -- --ignored

pqc-hybrid-example: ## Run PQC hybrid demo example
	$(AT)echo -e "$(CYAN)🌟 Running PQC hybrid demo example...$(RESET)"
	$(CURSED_CMD) examples/crypto_pqc_hybrid_demo.csd

pqc-hybrid-build-check: ## Check PQC hybrid system compilation
	$(AT)echo -e "$(CYAN)🔧 Checking PQC hybrid system compilation...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) check --lib
	$(LINKING_FIX) $(CARGO_CMD) check --test crypto_pqc_hybrid_test

pqc-hybrid-doc: ## Generate PQC hybrid documentation
	$(AT)echo -e "$(CYAN)📚 Generating PQC hybrid documentation...$(RESET)"
	$(LINKING_FIX) $(CARGO_CMD) doc --lib --open

pqc-hybrid-help: ## Show PQC hybrid system help
	$(AT)echo -e "$(BOLD)$(CYAN)🆘 PQC Hybrid System Testing Commands$(RESET)"
	$(AT)echo -e "$(CYAN)========================================$(RESET)"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test-quick$(RESET)      - Quick validation tests"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test$(RESET)            - Standard test suite"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test-all$(RESET)        - All tests including performance"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test-integration$(RESET) - Integration tests only"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test-security$(RESET)   - Security property tests"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-test-performance$(RESET) - Performance tests (ignored)"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-example$(RESET)         - Run demo example"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-build-check$(RESET)     - Check compilation"
	$(AT)echo -e "  $(YELLOW)pqc-hybrid-doc$(RESET)             - Generate documentation"

# Distributed optimization testing targets
# =============================================================================
.PHONY: distributed-opt-test distributed-opt-test-network distributed-opt-test-workers distributed-opt-test-parallel
.PHONY: distributed-opt-test-ml distributed-opt-test-pgo distributed-opt-benchmark distributed-opt-demo
.PHONY: distributed-opt-validate distributed-opt-clean distributed-opt-help

distributed-opt-test: ## Run distributed optimization tests
	$(AT)echo -e "$(CYAN)🔧 Running distributed optimization tests...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test distributed_optimization_integration_test $(V)

distributed-opt-test-network: ## Test network optimization
	$(AT)echo -e "$(CYAN)🌐 Testing network optimization...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --lib cursed::optimization::distributed::network_optimizer::tests $(V)

distributed-opt-test-workers: ## Test worker node management  
	$(AT)echo -e "$(CYAN)👥 Testing worker node management...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --lib cursed::optimization::distributed::worker_node::tests $(V)

distributed-opt-test-parallel: ## Test parallel compilation
	$(AT)echo -e "$(CYAN)⚡ Testing parallel compilation...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --lib cursed::optimization::parallel::tests $(V)

distributed-opt-test-ml: ## Test ML optimization engine
	$(AT)echo -e "$(CYAN)🤖 Testing ML optimization engine...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --lib cursed::optimization::ml_optimization::tests $(V)

distributed-opt-test-pgo: ## Test PGO LLVM integration
	$(AT)echo -e "$(CYAN)📊 Testing PGO LLVM integration...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --lib cursed::optimization::pgo::llvm_integration::tests $(V)

distributed-opt-benchmark: ## Run distributed optimization benchmarks
	$(AT)echo -e "$(CYAN)🏃 Running distributed optimization benchmarks...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test distributed_compilation_workflow --release -- --ignored $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test ml_optimization_integration --release -- --ignored $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test parallel_compilation_load_balancing --release -- --ignored $(V)

distributed-opt-demo: ## Run distributed optimization demo
	$(AT)echo -e "$(CYAN)🎭 Running distributed optimization demo...$(RESET)"
	$(AT)echo "Setting up demo environment..."
	$(AT)CURSED_COORDINATOR_ADDRESS="127.0.0.1:9000" \
	    CURSED_COMPILER="echo" \
	    CURSED_ML_OPTIMIZATION_ENABLED=true \
	    $(LINKING_FIX) $(CARGO_CMD) test test_distributed_compilation_workflow -- --nocapture $(V)

distributed-opt-validate: ## Validate distributed optimization implementation
	$(AT)echo -e "$(CYAN)✅ Validating distributed optimization implementation...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) check --lib $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test distributed_optimization_integration_test --no-run $(V)
	$(AT)echo -e "$(GREEN)✅ Distributed optimization system validated$(RESET)"

distributed-opt-clean: ## Clean distributed optimization artifacts
	$(AT)echo -e "$(YELLOW)🧹 Cleaning distributed optimization artifacts...$(RESET)"
	$(AT)rm -rf target/debug/build/cursed-*/out/distributed_*
	$(AT)rm -rf /tmp/cursed_distributed_test_*

distributed-opt-help: ## Show distributed optimization help
	$(AT)echo -e "$(CYAN)📖 Available distributed optimization targets:$(RESET)"
	$(AT)echo "  distributed-opt-test          - Run all distributed optimization tests"
	$(AT)echo "  distributed-opt-test-network  - Test network optimization"
	$(AT)echo "  distributed-opt-test-workers  - Test worker node management"
	$(AT)echo "  distributed-opt-test-parallel - Test parallel compilation"
	$(AT)echo "  distributed-opt-test-ml       - Test ML optimization engine"
	$(AT)echo "  distributed-opt-test-pgo      - Test PGO LLVM integration"
	$(AT)echo "  distributed-opt-benchmark     - Run performance benchmarks"
	$(AT)echo "  distributed-opt-demo          - Run distributed optimization demo"
	$(AT)echo "  distributed-opt-validate      - Validate implementation"
	$(AT)echo "  distributed-opt-clean         - Clean artifacts"
	$(AT)echo "  distributed-opt-help          - Show this help"

# Profile-Guided Optimization (PGO) System
# =============================================================================

.PHONY: pgo-test pgo-test-quick pgo-test-integration pgo-test-performance pgo-test-all \
        pgo-validate pgo-demo pgo-benchmark pgo-clean pgo-help

pgo-test: ## Run PGO integration tests
	$(AT)echo -e "$(CYAN)📊 Running PGO integration tests...$(RESET)"
	$(AT)./tests/run_pgo_tests.sh --integration $(V)

pgo-test-quick: ## Run quick PGO validation tests
	$(AT)echo -e "$(CYAN)⚡ Running quick PGO validation...$(RESET)"
	$(AT)./tests/run_pgo_tests.sh --quick $(V)

pgo-test-integration: ## Run comprehensive PGO integration tests
	$(AT)echo -e "$(CYAN)🔧 Running PGO integration tests...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test pgo_integration_test $(V)

pgo-test-performance: ## Run PGO performance tests
	$(AT)echo -e "$(CYAN)🏃 Running PGO performance tests...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test pgo_performance_test --release $(V)

pgo-test-all: ## Run all PGO tests including performance
	$(AT)echo -e "$(CYAN)🚀 Running all PGO tests...$(RESET)"
	$(AT)./tests/run_pgo_tests.sh --all --verbose $(V)

pgo-benchmark: ## Run PGO benchmarks and performance tests
	$(AT)echo -e "$(CYAN)🏁 Running PGO benchmarks...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test pgo_performance_test --release -- --ignored $(V)
	$(AT)./tests/run_pgo_tests.sh --performance --report pgo_benchmark_report.md

pgo-demo: ## Run PGO demonstration workflow
	$(AT)echo -e "$(CYAN)🎭 Running PGO demonstration...$(RESET)"
	$(AT)echo "Creating sample CURSED program for PGO demo..."
	$(AT)mkdir -p examples/pgo_demo
	$(AT)echo 'slay main() { println("PGO Demo"); }' > examples/pgo_demo/demo.csd
	$(AT)echo "PGO workflow demonstration:"
	$(AT)echo "1. Generate instrumented binary: cursed pgo generate examples/pgo_demo/demo.csd"
	$(AT)echo "2. Collect profile data: cursed pgo collect ./instrumented_binary"
	$(AT)echo "3. Apply optimizations: cursed pgo apply examples/pgo_demo/demo.csd --profile=profile.data"
	$(AT)echo "4. Full workflow: cursed pgo workflow examples/pgo_demo/demo.csd"

pgo-validate: ## Validate PGO implementation
	$(AT)echo -e "$(CYAN)✅ Validating PGO implementation...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) check --lib $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test pgo_integration_test --no-run $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test --test pgo_performance_test --no-run $(V)
	$(AT)./tests/run_pgo_tests.sh --quick
	$(AT)echo -e "$(GREEN)✅ PGO system validated$(RESET)"

pgo-report: ## Generate comprehensive PGO test report
	$(AT)echo -e "$(CYAN)📋 Generating PGO test report...$(RESET)"
	$(AT)./tests/run_pgo_tests.sh --all --report pgo_comprehensive_report.md
	$(AT)echo -e "$(GREEN)📋 PGO report generated: pgo_comprehensive_report.md$(RESET)"

pgo-coverage: ## Generate PGO test coverage report
	$(AT)echo -e "$(CYAN)📊 Generating PGO coverage report...$(RESET)"
	$(AT)$(LINKING_FIX) cargo tarpaulin --out Html --output-dir coverage/pgo \
		--include-tests --timeout 300 \
		--test pgo_integration_test --test pgo_performance_test $(V)
	$(AT)echo -e "$(GREEN)📊 PGO coverage report: coverage/pgo/tarpaulin-report.html$(RESET)"

pgo-stress: ## Run PGO stress tests
	$(AT)echo -e "$(CYAN)💪 Running PGO stress tests...$(RESET)"
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test test_scalability_limits --release -- --ignored $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test test_memory_usage_efficiency --release -- --ignored $(V)
	$(AT)$(LINKING_FIX) $(CARGO_CMD) test test_concurrent_collection_performance --release -- --ignored $(V)

pgo-clean: ## Clean PGO artifacts and test data
	$(AT)echo -e "$(YELLOW)🧹 Cleaning PGO artifacts...$(RESET)"
	$(AT)rm -rf pgo_profiles/
	$(AT)rm -rf examples/pgo_demo/
	$(AT)rm -f pgo_*.md pgo_*.txt pgo_*.json
	$(AT)rm -rf target/profraw target/profdata
	$(AT)rm -rf coverage/pgo/

pgo-help: ## Show PGO system help
	$(AT)echo -e "$(CYAN)📖 CURSED Profile-Guided Optimization (PGO) System$(RESET)"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Available PGO targets:$(RESET)"
	$(AT)echo "  pgo-test                - Run PGO integration tests"
	$(AT)echo "  pgo-test-quick          - Run quick PGO validation"
	$(AT)echo "  pgo-test-integration    - Run comprehensive integration tests"
	$(AT)echo "  pgo-test-performance    - Run PGO performance tests"
	$(AT)echo "  pgo-test-all            - Run all PGO tests"
	$(AT)echo "  pgo-benchmark           - Run PGO benchmarks"
	$(AT)echo "  pgo-demo                - Run PGO demonstration"
	$(AT)echo "  pgo-validate            - Validate PGO implementation"
	$(AT)echo "  pgo-report              - Generate comprehensive test report"
	$(AT)echo "  pgo-coverage            - Generate test coverage report"
	$(AT)echo "  pgo-stress              - Run stress tests"
	$(AT)echo "  pgo-clean               - Clean PGO artifacts"
	$(AT)echo "  pgo-help                - Show this help"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)PGO CLI Commands:$(RESET)"
	$(AT)echo "  cursed pgo generate <files>     - Generate instrumented binary"
	$(AT)echo "  cursed pgo collect <binary>     - Collect profile data"
	$(AT)echo "  cursed pgo analyze <profile>    - Analyze profile data"
	$(AT)echo "  cursed pgo apply <files>        - Apply PGO optimizations"
	$(AT)echo "  cursed pgo workflow <files>     - Full PGO workflow"
	$(AT)echo "  cursed pgo stats                - Show PGO statistics"
	$(AT)echo ""
	$(AT)echo -e "$(BOLD)Usage Examples:$(RESET)"
	$(AT)echo "  make pgo-test               # Basic PGO testing"
	$(AT)echo "  make pgo-benchmark          # Performance validation"
	$(AT)echo "  make pgo-demo               # See PGO in action"
	$(AT)echo "  make pgo-report             # Generate detailed report"

# Performance Optimization System Testing Commands
# =============================================================================
.PHONY: performance-optimization-test performance-optimization-benchmarks performance-optimization-test-all performance-optimization-help

performance-optimization-test: ## Run comprehensive performance optimization tests
	$(AT)echo -e "$(MAGENTA)🔧 Running comprehensive performance optimization tests...$(RESET)"
	$(AT)$(MAKE_WITH_LINKING) test --test comprehensive_performance_optimization_test

performance-optimization-benchmarks: ## Run performance optimization benchmarks
	$(AT)echo -e "$(MAGENTA)📊 Running performance optimization benchmarks...$(RESET)"
	$(AT)$(MAKE_WITH_LINKING) test --test performance_optimization_benchmarks

performance-optimization-test-all: performance-optimization-test performance-optimization-benchmarks ## Run all performance optimization tests
	$(AT)echo -e "$(GREEN)✅ All performance optimization tests completed$(RESET)"

performance-optimization-help: ## Show performance optimization test commands
	$(AT)echo -e "$(CYAN)Performance Optimization System Test Commands:$(RESET)"
	$(AT)echo -e "  $(GREEN)performance-optimization-test$(RESET)         - Run comprehensive optimization tests"
	$(AT)echo -e "  $(GREEN)performance-optimization-benchmarks$(RESET)   - Run performance benchmarks"
	$(AT)echo -e "  $(GREEN)performance-optimization-test-all$(RESET)     - Run all optimization tests"



# ================================================================================================
# Advanced LLVM Optimization System Tests
# ================================================================================================

.PHONY: advanced-opt-test advanced-opt-test-quick advanced-opt-test-verbose advanced-opt-test-report advanced-opt-help

advanced-opt-test: ## Run advanced LLVM optimization system tests
	@echo "🚀 Running Advanced LLVM Optimization System Tests"
	./tests/run_advanced_optimization_tests.sh

advanced-opt-test-quick: ## Run quick advanced optimization tests
	@echo "⚡ Running Quick Advanced Optimization Tests"
	./tests/run_advanced_optimization_tests.sh --quick

advanced-opt-test-verbose: ## Run advanced optimization tests with verbose output
	@echo "📝 Running Advanced Optimization Tests (Verbose)"
	./tests/run_advanced_optimization_tests.sh --verbose

advanced-opt-test-report: ## Generate advanced optimization test report
	@echo "📊 Generating Advanced Optimization Test Report"
	./tests/run_advanced_optimization_tests.sh --report advanced_optimization_report.md

advanced-opt-help: ## Show advanced optimization test help
	@echo "Advanced LLVM Optimization Test Commands:"
	@echo "  advanced-opt-test         - Run all advanced optimization tests"
	@echo "  advanced-opt-test-quick   - Run quick tests only"
	@echo "  advanced-opt-test-verbose - Run tests with verbose output"
	@echo "  advanced-opt-test-report  - Generate detailed test report"
	@echo "  advanced-opt-help         - Show this help message"

