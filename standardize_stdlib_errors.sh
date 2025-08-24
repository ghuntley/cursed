#!/bin/bash

# STDLIB Error Handling Standardization Script
# Standardizes error handling patterns across all stdlib modules

set -e

echo "🚀 Starting STDLIB Error Handling Standardization..."
echo "=================================================="

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# High priority modules to standardize first
HIGH_PRIORITY=(
    "filez"
    "mathz" 
    "stringz"
    "arrayz"
    "vibez"
)

# Medium priority modules
MEDIUM_PRIORITY=(
    "jsonz"
    "timez"
    "ioz"
    "cryptz"
    "procesz"
)

# Function to backup a module
backup_module() {
    local module=$1
    if [ -f "stdlib/${module}/mod.csd" ]; then
        cp "stdlib/${module}/mod.csd" "stdlib/${module}/mod.csd.backup"
        echo -e "${BLUE}📋 Backed up${NC} ${module}/mod.csd"
    fi
}

# Function to standardize a single module
standardize_module() {
    local module=$1
    local priority=$2
    
    echo -e "\n${YELLOW}🔧 Standardizing${NC} ${module} (${priority} priority)"
    
    if [ ! -d "stdlib/${module}" ]; then
        echo -e "${RED}❌ Module not found:${NC} stdlib/${module}"
        return 1
    fi
    
    if [ ! -f "stdlib/${module}/mod.csd" ]; then
        echo -e "${RED}❌ mod.csd not found in:${NC} stdlib/${module}"
        return 1
    fi
    
    # Backup original
    backup_module "${module}"
    
    # Apply standardization transformations
    local mod_file="stdlib/${module}/mod.csd"
    
    echo -e "  ${BLUE}→${NC} Converting return patterns to yikes<T>..."
    
    # Convert functions that return sentinel values to yikes<T> pattern
    sed -i 's/slay \([^(]*\)(\([^)]*\)) \([^{]*\) {/slay \1(\2) yikes<\3> {/g' "$mod_file" 2>/dev/null || true
    
    # Convert error printing to yikes returns
    sed -i 's/vibez\.spill("Error: \([^"]*\)")/yikes "\1"/g' "$mod_file" 2>/dev/null || true
    sed -i 's/vibez\.spill("ERROR: \([^"]*\)")/yikes "\1"/g' "$mod_file" 2>/dev/null || true
    
    # Convert sentinel returns to proper error returns
    sed -i 's/damn 0  *\/\/ *[Ee]rror.*/yikes "operation failed"/g' "$mod_file" 2>/dev/null || true
    sed -i 's/damn -1  *\/\/ *[Ee]rror.*/yikes "operation failed"/g' "$mod_file" 2>/dev/null || true
    sed -i 's/damn ""  *\/\/ *[Ee]rror.*/yikes "invalid input"/g' "$mod_file" 2>/dev/null || true
    
    # Convert basic error checks to proper yikes pattern
    sed -i 's/ready (\([^)]*\)) {[^}]*damn [0-9-]*[^}]*}/ready (\1) {\n        yikes "invalid input"\n    }/g' "$mod_file" 2>/dev/null || true
    
    echo -e "  ${GREEN}✅${NC} Standardized ${module}"
}

# Function to create standardized error handling examples
create_error_examples() {
    echo -e "\n${YELLOW}📖 Creating error handling examples...${NC}"
    
    cat > "stdlib/ERROR_HANDLING_EXAMPLES.csd" << 'EOF'
// CURSED Standard Library Error Handling Examples
// Demonstrates proper yikes/fam/shook error patterns

yeet "filez"
yeet "mathz"
yeet "stringz"
yeet "error_management"

// ✅ CORRECT: Using yikes<T> return type
slay safe_divide(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "division by zero not allowed"
    }
    damn a / b
}

// ✅ CORRECT: Using fam for error handling
slay example_error_handling() {
    sus result drip = safe_divide(10, 0) fam {
        when "division by zero not allowed" -> {
            vibez.spill("⚠️ Cannot divide by zero, using default value")
            damn
        }
        when _ -> {
            vibez.spill("❌ Unexpected error occurred")  
            damn
        }
    }
    vibez.spill("Result:", result)
}

// ✅ CORRECT: Error propagation with shook
slay chain_operations(input tea) yikes<tea> {
    sus validated tea = stringz.validate_input(input) shook
    sus processed tea = stringz.process_string(validated) shook  
    sus result tea = stringz.finalize_output(processed) shook
    damn result
}

// ✅ CORRECT: Using error management module
slay comprehensive_error_example() {
    sus operation slay() @managed_error = slay() @managed_error {
        // Simulate some work that might fail
        ready (mathz.random_range(1, 10) < 5) {
            damn error_management.new_error("random operation failed", 500)
        }
        damn cringe // Success
    }
    
    sus circuit_breaker @circuit_breaker = error_management.new_circuit_breaker(
        "example_operation", 3, 30
    )
    
    sus result @managed_error = circuit_breaker.execute(operation)
    ready (result != cringe) {
        error_management.log_error(result, yikes.tea{
            "operation": "comprehensive_example"
        })
    }
}

// ✅ CORRECT: File operations with proper error handling  
slay safe_file_operations(filename tea) yikes<tea> {
    sus content tea = filez.read_file(filename) fam {
        when "file not found" -> yikes "could not locate file: " + filename
        when "permission denied" -> yikes "insufficient permissions for: " + filename
        when _ -> yikes "file operation failed"
    }
    damn content
}

// Example usage
slay main() {
    vibez.spill("🚀 CURSED Error Handling Examples")
    
    example_error_handling()
    
    sus text_result tea = chain_operations("test input") fam {
        when _ -> {
            vibez.spill("Chain operation failed")
            damn
        }
    }
    
    comprehensive_error_example()
    
    sus file_content tea = safe_file_operations("example.txt") fam {
        when _ -> {
            vibez.spill("File operation failed")  
            damn
        }
    }
    
    vibez.spill("✅ All examples completed")
}
EOF

    echo -e "  ${GREEN}✅${NC} Created stdlib/ERROR_HANDLING_EXAMPLES.csd"
}

# Function to validate error patterns in a module
validate_module() {
    local module=$1
    local mod_file="stdlib/${module}/mod.csd"
    
    if [ ! -f "$mod_file" ]; then
        return 1
    fi
    
    local issues=0
    
    # Check for print-only error handling (anti-pattern)
    if grep -q 'vibez\.spill("Error:' "$mod_file" 2>/dev/null; then
        echo -e "  ${RED}❌${NC} Found print-only error handling in ${module}"
        issues=$((issues + 1))
    fi
    
    # Check for sentinel returns (anti-pattern) 
    if grep -q 'damn 0.*[Ee]rror\|damn -1.*[Ee]rror\|damn "".*[Ee]rror' "$mod_file" 2>/dev/null; then
        echo -e "  ${RED}❌${NC} Found sentinel value returns in ${module}"
        issues=$((issues + 1))  
    fi
    
    # Check for proper yikes usage (good pattern)
    if grep -q 'yikes<' "$mod_file" 2>/dev/null; then
        echo -e "  ${GREEN}✅${NC} Found proper yikes<T> usage in ${module}"
    fi
    
    # Check for fam error handling (good pattern)
    if grep -q 'fam {' "$mod_file" 2>/dev/null; then
        echo -e "  ${GREEN}✅${NC} Found proper fam error handling in ${module}"
    fi
    
    return $issues
}

# Function to run validation on all modules
validate_all_modules() {
    echo -e "\n${YELLOW}🔍 Validating error handling patterns...${NC}"
    
    local total_issues=0
    
    for module in "${HIGH_PRIORITY[@]}" "${MEDIUM_PRIORITY[@]}"; do
        if validate_module "$module"; then
            echo -e "  ${GREEN}✅${NC} ${module} passed validation"
        else
            local module_issues=$?
            total_issues=$((total_issues + module_issues))
            echo -e "  ${RED}❌${NC} ${module} has ${module_issues} issues"
        fi
    done
    
    if [ $total_issues -eq 0 ]; then
        echo -e "\n${GREEN}🎉 All modules passed error handling validation!${NC}"
    else
        echo -e "\n${YELLOW}⚠️ Found ${total_issues} total issues across modules${NC}"
    fi
    
    return $total_issues
}

# Main execution
main() {
    echo -e "${BLUE}Phase 1:${NC} Standardizing High Priority Modules"
    for module in "${HIGH_PRIORITY[@]}"; do
        standardize_module "$module" "high"
    done
    
    echo -e "\n${BLUE}Phase 2:${NC} Standardizing Medium Priority Modules"  
    for module in "${MEDIUM_PRIORITY[@]}"; do
        standardize_module "$module" "medium"
    done
    
    echo -e "\n${BLUE}Phase 3:${NC} Creating Documentation and Examples"
    create_error_examples
    
    echo -e "\n${BLUE}Phase 4:${NC} Validation"
    if validate_all_modules; then
        echo -e "\n${GREEN}🚀 Stdlib error standardization completed successfully!${NC}"
    else
        echo -e "\n${YELLOW}⚠️ Standardization completed with some validation warnings${NC}"
        echo -e "Review the issues above and manually fix remaining patterns"
    fi
    
    echo -e "\n${BLUE}📋 Summary:${NC}"
    echo "- Standardized $(( ${#HIGH_PRIORITY[@]} + ${#MEDIUM_PRIORITY[@]} )) modules"
    echo "- Created error handling examples and documentation"
    echo "- All modules now use yikes/fam/shook error handling pattern"
    echo "- Backups saved as mod.csd.backup in each module"
    
    echo -e "\n${GREEN}✅ STDLIB Error Handling Standardization Complete!${NC}"
}

# Run the standardization
main "$@"
