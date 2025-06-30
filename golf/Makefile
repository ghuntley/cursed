# CURSED Rule 30 Code Golf Makefile

CURSED_COMPILER = ../target/debug/cursed

# Colors
RESET := \033[0m
BOLD := \033[1m
GREEN := \033[32m
BLUE := \033[34m
YELLOW := \033[33m
CYAN := \033[36m

.PHONY: help demo golf test count clean

help: ## Show available targets
	@echo -e "$(BOLD)CURSED Rule 30 Code Golf$(RESET)"
	@echo -e "$(CYAN)Available targets:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-15s$(RESET) %s\n", $$1, $$2}'

demo: ## Run the demonstration version
	@echo -e "$(CYAN)🎮 Running Rule 30 Demo...$(RESET)"
	$(CURSED_COMPILER) simple_demo.csd

simple: ## Run the simple version
	@echo -e "$(CYAN)🎯 Running Simple Rule 30...$(RESET)"
	$(CURSED_COMPILER) simple_rule30.csd

golf: ## Run the golf version (shortest code)
	@echo -e "$(CYAN)⛳ Running Golf Version...$(RESET)"
	@echo -e "$(BLUE)Source code:$(RESET)"
	@cat shortest.csd
	@echo -e "$(BLUE)Output:$(RESET)"
	$(CURSED_COMPILER) shortest.csd

test: ## Test golf version with all n values [1-12]
	@echo -e "$(CYAN)🧪 Testing all n values [1-12]...$(RESET)"
	@for n in {1..12}; do \
		echo -e "$(BLUE)Testing n=$$n:$(RESET)"; \
		sed "s/sus n=1;/sus n=$$n;/" golf_final.csd > temp_test.csd; \
		$(CURSED_COMPILER) temp_test.csd; \
		echo ""; \
	done
	@rm -f temp_test.csd

count: ## Count bytes in golf version
	@echo -e "$(CYAN)📏 Counting bytes in golf version...$(RESET)"
	@echo -e "$(BLUE)File: shortest.csd$(RESET)"
	@wc -c shortest.csd
	@echo -e "$(BLUE)Code without newlines:$(RESET)"
	@tr -d '\n' < shortest.csd | wc -c

micro: ## Run the ultra-minimal version
	@echo -e "$(CYAN)🎯 Running Micro Version (21 bytes)...$(RESET)"
	@echo -e "$(BLUE)Source:$(RESET)"
	@cat micro.csd
	@echo -e "$(BLUE)Output:$(RESET)"
	$(CURSED_COMPILER) micro.csd

clean: ## Clean temporary files
	@echo -e "$(CYAN)🧹 Cleaning temporary files...$(RESET)"
	@rm -f temp_test.csd
	@echo -e "$(GREEN)✅ Clean completed$(RESET)"

info: ## Show challenge information
	@echo -e "$(BOLD)CURSED Rule 30 Code Golf Challenge$(RESET)"
	@echo ""
	@echo -e "$(BLUE)Challenge:$(RESET)"
	@echo "- Input n ∈ [1‒12]"
	@echo "- Treat program bytes as circular binary tape"  
	@echo "- Evolve 1-D Rule 30 for n steps"
	@echo "- Print final tape in hex (no 0x)"
	@echo "- Same code must run for any n"
	@echo "- Fewest bytes wins"
	@echo ""
	@echo -e "$(BLUE)Rule 30:$(RESET)"
	@echo "111→0, 110→0, 101→0, 100→1"
	@echo "011→1, 010→1, 001→1, 000→0"
	@echo "Formula: new_cell = left XOR (center OR right)"
	@echo ""
	@echo -e "$(BLUE)Files:$(RESET)"
	@echo "- golf_final.csd: Shortest version (122 bytes)"
	@echo "- micro.csd: Ultra-minimal (21 bytes)"
	@echo "- demo_rule30.csd: Full demonstration"
	@echo "- simple_rule30.csd: Educational version"

benchmark: golf ## Benchmark the golf version
	@echo -e "$(CYAN)⏱️  Benchmarking golf version...$(RESET)"
	@time $(CURSED_COMPILER) golf_final.csd

.DEFAULT_GOAL := help
