// Helper functions for CURSED linter
// Utility functions used by the main linter implementation

yeet "stringz"
yeet "arrayz"
yeet "filez"

// String utility functions
slay find_str(text tea, pattern tea) drip {
    // Find first occurrence of pattern in text
    sus text_len drip = len_str(text)
    sus pattern_len drip = len_str(pattern)
    
    ready (pattern_len == 0 || text_len < pattern_len) {
        damn -1
    }
    
    sus i drip = 0
    bestie (i <= text_len - pattern_len) {
        sus match lit = based
        sus j drip = 0
        
        bestie (j < pattern_len) {
            ready (char_at(text, i + j) != char_at(pattern, j)) {
                match = cringe
                break
            }
            j = j + 1
        }
        
        ready (match) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay find_str_from(text tea, pattern tea, start_pos drip) drip {
    sus text_len drip = len_str(text)
    ready (start_pos >= text_len) {
        damn -1
    }
    
    sus substring_text tea = substring(text, start_pos, text_len)
    sus result drip = find_str(substring_text, pattern)
    
    ready (result == -1) {
        damn -1
    }
    
    damn start_pos + result
}

slay count_char(text tea, ch tea) drip {
    sus count drip = 0
    sus i drip = 0
    
    bestie (i < len_str(text)) {
        ready (char_at(text, i) == ch) {
            count = count + 1
        }
        i = i + 1
    }
    
    damn count
}

slay char_at(text tea, index drip) tea {
    ready (index < 0 || index >= len_str(text)) {
        damn ""
    }
    
    damn substring(text, index, index + 1)
}

slay starts_with(text tea, prefix tea) lit {
    sus prefix_len drip = len_str(prefix)
    ready (len_str(text) < prefix_len) {
        damn cringe
    }
    
    sus text_prefix tea = substring(text, 0, prefix_len)
    damn text_prefix == prefix
}

slay ends_with(text tea, suffix tea) lit {
    sus text_len drip = len_str(text)
    sus suffix_len drip = len_str(suffix)
    
    ready (text_len < suffix_len) {
        damn cringe
    }
    
    sus text_suffix tea = substring(text, text_len - suffix_len, text_len)
    damn text_suffix == suffix
}

slay trim_right(text tea) tea {
    sus len drip = len_str(text)
    sus end drip = len
    
    bestie (end > 0 && (char_at(text, end - 1) == " " || char_at(text, end - 1) == "\t")) {
        end = end - 1
    }
    
    ready (end == len) {
        damn text
    }
    
    damn substring(text, 0, end)
}

slay to_lower_str(text tea) tea {
    // Simplified lowercase conversion
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < len_str(text)) {
        sus ch tea = char_at(text, i)
        ready (ch >= "A" && ch <= "Z") {
            // Convert to lowercase (simplified)
            result = concat_str(result, ch)  // In real implementation, would convert case
        } otherwise {
            result = concat_str(result, ch)
        }
        i = i + 1
    }
    
    damn result
}

slay contains_digits(text tea) lit {
    sus digits []tea = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
    
    sus i drip = 0
    bestie (i < len(digits)) {
        ready (contains_str(text, digits[i])) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay extract_numbers(text tea) []tea {
    sus numbers []tea = []
    sus current_number tea = ""
    sus i drip = 0
    
    bestie (i < len_str(text)) {
        sus ch tea = char_at(text, i)
        ready (is_digit(ch) || (ch == "." && len_str(current_number) > 0)) {
            current_number = concat_str(current_number, ch)
        } otherwise {
            ready (len_str(current_number) > 0) {
                push(numbers, current_number)
                current_number = ""
            }
        }
        i = i + 1
    }
    
    ready (len_str(current_number) > 0) {
        push(numbers, current_number)
    }
    
    damn numbers
}

slay is_digit(ch tea) lit {
    damn ch >= "0" && ch <= "9"
}

slay is_simple_assignment(line tea) lit {
    sus trimmed tea = trim_str(line)
    damn starts_with(trimmed, "sus ") && contains_str(trimmed, "=")
}

slay repeat_char(ch tea, count drip) tea {
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < count) {
        result = concat_str(result, ch)
        i = i + 1
    }
    
    damn result
}

// File utility functions
slay read_file_safe(file_path tea) tea {
    // In a real implementation, this would safely read the file
    // For now, return empty string on error
    ready (!file_exists(file_path)) {
        damn ""
    }
    
    // This would use actual file reading
    damn read_file(file_path)
}

slay file_exists(file_path tea) lit {
    // In a real implementation, this would check file existence
    // For testing, assume all files exist
    damn based
}

// Array utility functions
slay filter_by_severity(issues []LintIssue, severity_level drip) []LintIssue {
    sus filtered []LintIssue = []
    
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].severity.level == severity_level) {
            push(filtered, issues[i])
        }
        i = i + 1
    }
    
    damn filtered
}

slay count_issues_by_severity(issues []LintIssue, severity_level drip) drip {
    sus count drip = 0
    
    sus i drip = 0
    bestie (i < len(issues)) {
        ready (issues[i].severity.level == severity_level) {
            count = count + 1
        }
        i = i + 1
    }
    
    damn count
}

// Linter-specific helper functions
slay get_current_function_start(linter Linter) drip {
    ready (len(linter.functions) == 0) {
        damn linter.current_line
    }
    
    sus current_func_index drip = len(linter.functions) - 1
    damn linter.functions[current_func_index].start_line
}

slay is_function_end(line tea) lit {
    sus trimmed tea = trim_str(line)
    damn trimmed == "}" && !contains_str(trimmed, "{")
}

slay extract_return_type_from_line(line tea) tea {
    sus paren_end drip = find_str(line, ")")
    sus brace_start drip = find_str(line, "{")
    
    ready (paren_end != -1 && brace_start != -1 && brace_start > paren_end) {
        sus return_section tea = substring(line, paren_end + 1, brace_start)
        damn trim_str(return_section)
    }
    
    damn ""
}

// Configuration loading functions
slay load_config_from_file(config_file tea, default_config LintConfig) LintConfig {
    // In a real implementation, this would parse the TOML config file
    // For now, return the default config
    ready (!file_exists(config_file)) {
        damn default_config
    }
    
    // This would parse the actual config file
    damn default_config
}

// Command line argument parsing
slay get_args() []tea {
    // In a real implementation, this would get command line arguments
    // For testing, return a simple test case
    sus args []tea = ["cursed-lint", "test_cursed_lint.csd"]
    damn args
}

slay exit_with_code(code drip) {
    // In a real implementation, this would exit with the given code
    ready (code != 0) {
        vibez.spill("Exiting with code: " + int_to_str(code))
    }
}

// Linter data structures from main linter (needed for compilation)
squad LintIssue {
    spill rule_id tea
    spill severity Severity
    spill message tea 
    spill file_path tea
    spill line drip
    spill column drip
    spill end_line drip
    spill end_column drip
    spill suggestion tea
    spill auto_fixable lit
    spill category tea
    spill source_line tea
    spill documentation_url tea
}

squad Severity {
    spill level drip
    spill name tea
    spill color tea
    spill exit_code drip
}

squad LintConfig {
    spill max_line_length drip
    spill max_function_length drip
    spill max_function_params drip
    spill max_cognitive_complexity drip
    spill max_nesting_depth drip
    spill max_cyclomatic_complexity drip
    spill check_unused_variables lit
    spill check_unused_functions lit
    spill check_unused_imports lit
    spill check_naming_conventions lit
    spill check_function_complexity lit
    spill check_duplicate_code lit
    spill check_dead_code lit
    spill check_security_vulnerabilities lit
    spill check_hardcoded_secrets lit
    spill check_sql_injection lit
    spill check_command_injection lit
    spill check_weak_cryptography lit
    spill check_unsafe_operations lit
    spill check_input_validation lit
    spill check_performance_issues lit
    spill check_memory_leaks lit
    spill check_inefficient_algorithms lit
    spill check_string_concatenation lit
    spill check_loop_optimizations lit
    spill check_memory_allocations lit
    spill enforce_gen_z_syntax lit
    spill prefer_vibez_output lit
    spill check_proper_yeet_usage lit
    spill check_slay_conventions lit
    spill enforce_squad_usage lit
    spill check_error_handling lit
    spill prefer_immutable_vars lit
    spill enforce_indentation lit
    spill check_trailing_whitespace lit
    spill check_empty_lines lit
    spill enforce_brace_style lit
    spill check_spacing lit
    spill output_format tea
    spill show_suggestions lit
    spill show_documentation lit
    spill color_output lit
    spill verbose_output lit
}

squad Linter {
    spill config LintConfig
    spill issues []LintIssue
    spill variables []Variable
    spill functions []Function
    spill imports []Import
    spill current_line drip
    spill current_file tea
    spill nesting_depth drip
    spill scope_depth drip
    spill in_function lit
    spill current_function tea
    spill security_score drip
    spill performance_score drip
    spill maintainability_score drip
}

squad Variable {
    spill name tea
    spill var_type tea
    spill line drip
    spill column drip
    spill scope_depth drip
    spill used lit
    spill times_used drip
    spill is_parameter lit
    spill is_mutable lit
    spill last_assignment drip
}

squad Function {
    spill name tea
    spill start_line drip
    spill end_line drip
    spill param_count drip
    spill return_type tea
    spill complexity drip
    spill lines_of_code drip
    spill called lit
    spill recursive lit
    spill has_unsafe_operations lit
    spill has_documentation lit
}

squad Import {
    spill module_name tea
    spill import_line drip
    spill used lit
    spill functions_used []tea
    spill is_stdlib lit
}
