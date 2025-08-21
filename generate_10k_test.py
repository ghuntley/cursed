#!/usr/bin/env python3
"""Generate 10K-line CURSED file for LSP testing"""

def generate_cursed_test_file():
    content = []
    
    # Header
    content.extend([
        "// 10K-line CURSED file for LSP server testing",
        "// Week 2 Oracle Tools Phase - Performance validation",
        "",
        "yeet \"vibez\";",
        "yeet \"mathz\";", 
        "yeet \"stringz\";",
        "yeet \"arrayz\";",
        "yeet \"testz\";",
        "",
        "slay main() {",
        "    vibez.spill(\"Starting 10K line LSP test\");",
        "",
    ])
    
    # Generate 9,900+ lines of diverse CURSED code
    for i in range(3300):
        # Variable declarations (different types)
        content.append(f"    sus var{i}_drip drip = {i};")
        content.append(f"    sus var{i}_tea tea = \"value_{i}\";") 
        content.append(f"    sus var{i}_lit lit = {('based' if i % 2 == 0 else 'cringe')};")
        
        # Conditional logic
        if i % 100 == 0:
            content.extend([
                f"    ready (var{i}_drip > {i//2}) {{",
                f"        vibez.spill(\"Checkpoint {i}\");",
                "    }",
            ])
            
        # Function calls every 50 iterations
        if i % 50 == 0:
            content.append(f"    sus result{i} drip = mathz.max(var{i}_drip, {i+1});")
            
        # Array operations every 25 iterations  
        if i % 25 == 0:
            content.append(f"    sus array{i} []drip = [1, 2, {i}, {i+1}];")
            
    # Loop structures
    content.extend([
        "",
        "    // Loop test section",
        "    bestie (i in 0..1000) {",
        "        ready (i % 100 == 0) {",
        "            vibez.spill(\"Loop progress:\", i);",
        "        }",
        "    }",
        "",
    ])
    
    # Error handling section
    content.extend([
        "    // Error handling test",
        "    sus safe_value drip = risky_calc(42) fam {",
        "        when \"overflow\" -> {",
        "            vibez.spill(\"Handled overflow\");", 
        "            damn 0;",
        "        }",
        "        when \"underflow\" -> {",
        "            vibez.spill(\"Handled underflow\");",
        "            damn 1;", 
        "        }",
        "    };",
        "",
    ])
    
    # Function definitions
    content.extend([
        "    vibez.spill(\"10K line test complete!\");",
        "}",
        "",
        "slay risky_calc(input drip) yikes<drip> {",
        "    ready (input > 1000) {",
        "        yikes \"overflow\";",
        "    }",
        "    ready (input < 0) {",
        "        yikes \"underflow\";", 
        "    }",
        "    damn input * 2;",
        "}",
        "",
    ])
    
    # Additional helper functions to reach 10K lines
    for i in range(100):
        content.extend([
            f"slay helper_func_{i}(param drip) drip {{",
            f"    sus local_var drip = param + {i};",
            f"    ready (local_var > {i*10}) {{",
            f"        damn local_var;",
            "    } otherwise {",
            f"        damn {i};",
            "    }",
            "}",
            "",
        ])
    
    # Final padding to ensure 10K+ lines
    content.extend([
        "// End of 10K-line CURSED LSP test file",
        f"// Total lines should be: {len(content) + 2}",
    ])
    
    return "\n".join(content)

if __name__ == "__main__":
    content = generate_cursed_test_file()
    with open("lsp_10k_test.csd", "w") as f:
        f.write(content)
    
    lines = content.count('\n') + 1
    print(f"Generated CURSED file with {lines} lines ({len(content)} bytes)")
    print("File saved as: lsp_10k_test.csd")
