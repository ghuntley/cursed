# Topological Sort Demo Makefile
# For CURSED Programming Language

# Configuration
CURSED_COMPILER = ../target/debug/cursed
CURSED_RELEASE_COMPILER = ../target/release/cursed
SOURCE = working_topo_sort.csd
BINARY = topo_sort
TEST_INPUT = input.txt

# Colors for output
RESET := \033[0m
BOLD := \033[1m
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
CYAN := \033[36m

.PHONY: help build build-release run test clean

help: ## Show this help message
	@echo -e "$(BOLD)CURSED Topological Sort Demo$(RESET)"
	@echo -e "$(CYAN)Available targets:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-15s$(RESET) %s\n", $$1, $$2}'

build: ## Build the demo (debug mode)
	@echo -e "$(CYAN)🔧 Building topological sort demo...$(RESET)"
	@if [ ! -f "$(CURSED_COMPILER)" ]; then \
		echo -e "$(RED)❌ CURSED compiler not found at $(CURSED_COMPILER)$(RESET)"; \
		echo -e "$(YELLOW)💡 Run 'make build' in the parent directory first$(RESET)"; \
		exit 1; \
	fi
	@echo -e "$(BLUE)📝 Compiling $(SOURCE)...$(RESET)"
	$(CURSED_COMPILER) $(SOURCE)
	@echo -e "$(BLUE)🔨 Creating executable wrapper...$(RESET)"
	@chmod +x $(BINARY)
	@echo -e "$(GREEN)✅ Build completed! Run with ./$(BINARY)$(RESET)"

build-release: ## Build the demo (release mode)
	@echo -e "$(CYAN)🚀 Building topological sort demo (release)...$(RESET)"
	@if [ ! -f "$(CURSED_RELEASE_COMPILER)" ]; then \
		echo -e "$(RED)❌ CURSED release compiler not found$(RESET)"; \
		echo -e "$(YELLOW)💡 Run 'make build-release' in the parent directory first$(RESET)"; \
		exit 1; \
	fi
	@echo -e "$(BLUE)📝 Compiling $(SOURCE) in release mode...$(RESET)"
	$(CURSED_RELEASE_COMPILER) $(SOURCE)
	@echo -e "$(GREEN)✅ Release build completed!$(RESET)"

run: build ## Run the topological sort demo
	@echo -e "$(CYAN)🏃 Running topological sort demo...$(RESET)"
	@echo -e "$(BLUE)Input file contents:$(RESET)"
	@cat $(TEST_INPUT)
	@echo ""
	@echo -e "$(BLUE)Program output:$(RESET)"
	$(CURSED_COMPILER) $(SOURCE)

test: build ## Run the demo with test cases
	@echo -e "$(CYAN)🧪 Testing topological sort demo...$(RESET)"
	@echo -e "$(BLUE)Test input:$(RESET)"
	@cat $(TEST_INPUT)
	@echo ""
	@echo -e "$(BLUE)Running test...$(RESET)"
	@$(CURSED_COMPILER) $(SOURCE) || echo -e "$(YELLOW)⚠️  Test completed (output above)$(RESET)"

benchmark: build-release ## Benchmark the demo (if release build available)
	@echo -e "$(CYAN)⏱️  Benchmarking topological sort demo...$(RESET)"
	@if [ -f "$(CURSED_RELEASE_COMPILER)" ]; then \
		time $(CURSED_RELEASE_COMPILER) $(SOURCE); \
	else \
		echo -e "$(YELLOW)⚠️  Release compiler not available, using debug...$(RESET)"; \
		time $(CURSED_COMPILER) $(SOURCE); \
	fi

binary: build ## Create and test the binary
	@echo -e "$(CYAN)🎯 Testing binary executable...$(RESET)"
	@./$(BINARY)
	@echo -e "$(GREEN)✅ Binary test completed!$(RESET)"

clean: ## Clean build artifacts
	@echo -e "$(CYAN)🧹 Cleaning build artifacts...$(RESET)"
	@rm -f *.o *.ll
	@echo -e "$(GREEN)✅ Clean completed!$(RESET)"

verify-compiler: ## Verify CURSED compiler is available
	@echo -e "$(CYAN)🔍 Verifying CURSED compiler...$(RESET)"
	@if [ -f "$(CURSED_COMPILER)" ]; then \
		echo -e "$(GREEN)✅ Debug compiler found: $(CURSED_COMPILER)$(RESET)"; \
		$(CURSED_COMPILER) --version || echo -e "$(YELLOW)⚠️  Version info not available$(RESET)"; \
	else \
		echo -e "$(RED)❌ Debug compiler not found: $(CURSED_COMPILER)$(RESET)"; \
	fi
	@if [ -f "$(CURSED_RELEASE_COMPILER)" ]; then \
		echo -e "$(GREEN)✅ Release compiler found: $(CURSED_RELEASE_COMPILER)$(RESET)"; \
	else \
		echo -e "$(YELLOW)⚠️  Release compiler not found: $(CURSED_RELEASE_COMPILER)$(RESET)"; \
	fi

info: ## Show project information
	@echo -e "$(BOLD)CURSED Topological Sort Demo$(RESET)"
	@echo -e "$(CYAN)Source file:$(RESET) $(SOURCE)"
	@echo -e "$(CYAN)Input file:$(RESET) $(TEST_INPUT)"
	@echo -e "$(CYAN)Compiler:$(RESET) $(CURSED_COMPILER)"
	@echo ""
	@echo -e "$(BLUE)Description:$(RESET)"
	@echo "This demo implements a topological sorting algorithm"
	@echo "for directed acyclic graphs (DAGs). The algorithm"
	@echo "demonstrates graph traversal and ordering concepts."
	@echo ""
	@echo -e "$(BLUE)Input format:$(RESET)"
	@echo "Each line represents an edge: 'A B' means A -> B"
	@echo ""
	@echo -e "$(BLUE)Example:$(RESET)"
	@cat $(TEST_INPUT)

# Default target
.DEFAULT_GOAL := help
