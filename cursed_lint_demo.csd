// CURSED Linter Demo - Standalone CLI Tool

yeet "stringz"

// Simple linter implementation
slay check_naming(line tea) tea {
    ready (contains_str(line, "sus ") && contains_str(line, "Variable")) {
        damn "⚠️  Variable should use snake_case naming";
    }
    damn "";
}

slay check_security(line tea) tea {
    ready (contains_str(line, "password") && contains_str(line, "\"")) {
        damn "🚨 Hardcoded password detected";
    }
    damn "";
}

slay check_gen_z(line tea) tea {
    ready (contains_str(line, "true")) {
        damn "✨ Use 'based' instead of 'true'";
    }
    damn "";
}

slay lint_simple(code tea) {
    vibez.spill("🔍 CURSED Linter Analysis");
    vibez.spill("========================");
    
    sus lines []tea = split_str(code, "\n");
    sus line_num drip = 1;
    sus issues_found drip = 0;
    
    bestie (line_num <= len(lines)) {
        sus line tea = lines[line_num - 1];
        
        // Check various issues
        sus naming_issue tea = check_naming(line);
        sus security_issue tea = check_security(line);
        sus gen_z_issue tea = check_gen_z(line);
        
        ready (len_str(naming_issue) > 0) {
            vibez.spill("Line " + int_to_str(line_num) + ": " + naming_issue);
            issues_found = issues_found + 1;
        }
        
        ready (len_str(security_issue) > 0) {
            vibez.spill("Line " + int_to_str(line_num) + ": " + security_issue);
            issues_found = issues_found + 1;
        }
        
        ready (len_str(gen_z_issue) > 0) {
            vibez.spill("Line " + int_to_str(line_num) + ": " + gen_z_issue);
            issues_found = issues_found + 1;
        }
        
        line_num = line_num + 1;
    }
    
    ready (issues_found == 0) {
        vibez.spill("✅ No issues found! Code is fire! 🔥");
    } otherwise {
        vibez.spill("📊 Total issues: " + int_to_str(issues_found));
    }
}

slay main() {
    vibez.spill("🔥 CURSED Linter Demo");
    vibez.spill("====================");
    
    // Test different code samples
    vibez.spill("");
    vibez.spill("Testing sample 1:");
    sus sample1 tea = "sus myVariable drip = 42";
    vibez.spill("Code: " + sample1);
    lint_simple(sample1);
    
    vibez.spill("");
    vibez.spill("Testing sample 2:");
    sus sample2 tea = "sus password tea = \"secret123\"";
    vibez.spill("Code: " + sample2);
    lint_simple(sample2);
    
    vibez.spill("");
    vibez.spill("Testing sample 3:");
    sus sample3 tea = "sus flag lit = true";
    vibez.spill("Code: " + sample3);
    lint_simple(sample3);
    
    vibez.spill("");
    vibez.spill("Testing sample 4 (clean code):");
    sus sample4 tea = "sus my_variable drip = based";
    vibez.spill("Code: " + sample4);
    lint_simple(sample4);
    
    vibez.spill("");
    vibez.spill("💯 CURSED Linter Demo Complete!");
}
