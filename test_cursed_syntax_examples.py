#!/usr/bin/env python3
"""
CURSED Syntax Examples Validator

This script creates and validates various CURSED syntax examples to test
the parser's conformance to the grammar specification.
"""

def create_syntax_examples():
    """Create comprehensive CURSED syntax examples based on grammar spec"""
    
    examples = {
        "basic_program": '''
vibe main

yeet "fmt"

slay main() {
    print("Hello, World!")
    yolo 0
}
''',

        "package_and_imports": '''
vibe my_package

yeet (
    "fmt"
    tea "strings"
    io "std::io"
)
''',

        "constants_and_variables": '''
facts (
    PI = 3.14159
    MAX_SIZE = 100
    VERSION = "1.0.0"
)

sus name tea = "Alice"
sus age, height = 25, 180.5
facts readonly_value = 42
''',

        "type_declarations": '''
be_like Person squad {
    name tea
    age normie
    email tea
}

be_like Counter collab {
    increment()
    decrement()
    value() normie
}
''',

        "function_declarations": '''
slay add(x, y normie) normie {
    yolo x + y
}

slay greet(name tea) {
    print("Hello, " + name)
}

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        yolo n
    }
    yolo fibonacci(n-1) + fibonacci(n-2)
}
''',

        "control_flow_if": '''
slay check_age(age normie) {
    lowkey age >= 18 {
        print("Adult")
    } highkey lowkey age >= 13 {
        print("Teenager")
    } highkey {
        print("Child")
    }
}

slay simple_check(x normie) {
    lowkey (x > 0) {
        print("Positive")
    }
}
''',

        "control_flow_switch": '''
slay process_day(day tea) {
    vibe_check day {
        mood "Monday", "Tuesday":
            print("Start of week")
        mood "Wednesday":
            print("Midweek")
        mood "Friday":
            print("End of week")
        basic:
            print("Weekend or other")
    }
}
''',

        "control_flow_loops": '''
slay loop_examples() {
    // For loop
    bestie i := 0; i < 10; i++ {
        print(i)
        lowkey i == 5 {
            ghosted  // break
        }
        lowkey i == 2 {
            simp     // continue
        }
    }
    
    // Range-based for
    sus items = [1, 2, 3, 4, 5]
    bestie _, value := flex items {
        print(value)
    }
    
    // While loop
    sus x = 10
    periodt x > 0 {
        print(x)
        x--
    }
    
    // Infinite loop
    bestie {
        print("Running...")
        lowkey should_stop() {
            ghosted
        }
    }
}
''',

        "expressions_and_operators": '''
slay expression_examples() {
    // Arithmetic
    sus result = (10 + 5) * 2 - 3 / 1
    
    // Comparisons
    sus is_equal = (x == y)
    sus is_greater = (a > b)
    sus is_less_equal = (c <= d)
    
    // Logical
    sus and_result = (x > 0) && (y < 10)
    sus or_result = (a == 1) || (b == 2)
    sus not_result = !(is_complete)
    
    // Member access
    sus person_name = person.name
    sus array_item = items[0]
    sus slice_part = data[1:5]
    
    // Function calls
    sus sum = add(10, 20)
    sus formatted = format_string("Hello {}", name)
}
''',

        "goroutines_and_channels": '''
slay concurrency_example() {
    // Channel creation
    sus ch dm normie = make(dm normie, 10)
    
    // Goroutine spawning
    stan producer(ch)
    stan consumer(ch)
    
    // Channel operations
    ch <- 42        // Send
    sus value = <-ch // Receive
    
    // Buffered channel
    sus buf_ch = make(dm tea, 5)
    buf_ch <- "message"
}

slay producer(ch dm normie) {
    bestie i := 0; i < 10; i++ {
        ch <- i
    }
    close(ch)
}

slay consumer(ch dm normie) {
    bestie value := flex ch {
        print("Received:", value)
    }
}
''',

        "error_handling": '''
slay file_operations() (tea, Error) {
    sus content, err = read_file("data.txt")
    lowkey err != cap {
        yolo "", err
    }
    
    sus processed = process_content(content)
    yolo processed, cap
}

slay safe_division(a, b normie) (normie, Error) {
    lowkey b == 0 {
        yolo 0, new_error("division by zero")
    }
    yolo a / b, cap
}
''',

        "complex_program": '''
vibe calculator

yeet (
    "fmt"
    "math"
)

be_like Operation squad {
    left normie
    right normie
    operator tea
}

collab Calculator {
    add(a, b normie) normie
    subtract(a, b normie) normie
    multiply(a, b normie) normie
    divide(a, b normie) (normie, Error)
}

be_like BasicCalculator squad {}

slay (calc *BasicCalculator) add(a, b normie) normie {
    yolo a + b
}

slay (calc *BasicCalculator) subtract(a, b normie) normie {
    yolo a - b
}

slay (calc *BasicCalculator) multiply(a, b normie) normie {
    yolo a * b
}

slay (calc *BasicCalculator) divide(a, b normie) (normie, Error) {
    lowkey b == 0 {
        yolo 0, new_error("division by zero")
    }
    yolo a / b, cap
}

slay main() {
    sus calc = &BasicCalculator{}
    
    sus operations = []Operation{
        {10, 5, "+"},
        {10, 5, "-"},
        {10, 5, "*"},
        {10, 5, "/"},
    }
    
    bestie _, op := flex operations {
        vibe_check op.operator {
            mood "+":
                sus result = calc.add(op.left, op.right)
                print("Result:", result)
            mood "-":
                sus result = calc.subtract(op.left, op.right)
                print("Result:", result)
            mood "*":
                sus result = calc.multiply(op.left, op.right)
                print("Result:", result)
            mood "/":
                sus result, err = calc.divide(op.left, op.right)
                lowkey err != cap {
                    print("Error:", err)
                } highkey {
                    print("Result:", result)
                }
            basic:
                print("Unknown operator:", op.operator)
        }
    }
}
'''
    }
    
    return examples

def save_examples():
    """Save all syntax examples to files"""
    examples = create_syntax_examples()
    
    for name, content in examples.items():
        filename = f"test_{name}.csd"
        with open(filename, 'w') as f:
            f.write(content.strip())
        print(f"Created: {filename}")

def analyze_syntax_coverage():
    """Analyze how well the examples cover the grammar specification"""
    
    # Grammar elements from specs/grammar.md
    grammar_elements = {
        "PackageClause": ["vibe"],
        "ImportDecl": ["yeet"],
        "ConstDecl": ["facts"],
        "VarDecl": ["sus"],
        "TypeDecl": ["be_like", "squad", "collab"],
        "FuncDecl": ["slay"],
        "IfStmt": ["lowkey", "highkey"],
        "SwitchStmt": ["vibe_check", "mood", "basic"],
        "ForStmt": ["bestie", "flex"],
        "WhileStmt": ["periodt"],
        "ReturnStmt": ["yolo"],
        "BreakStmt": ["ghosted"],
        "ContinueStmt": ["simp"],
        "GoStmt": ["stan"],
        "Expressions": ["operators", "function_calls", "member_access"],
        "Channels": ["dm", "<-", "make", "close"],
        "ErrorHandling": ["cap", "Error"],
    }
    
    examples = create_syntax_examples()
    all_content = " ".join(examples.values())
    
    coverage = {}
    
    for element, keywords in grammar_elements.items():
        covered_keywords = []
        for keyword in keywords:
            if keyword in all_content:
                covered_keywords.append(keyword)
        
        coverage[element] = {
            'total': len(keywords),
            'covered': len(covered_keywords),
            'keywords': covered_keywords,
            'percentage': (len(covered_keywords) / len(keywords)) * 100 if keywords else 0
        }
    
    return coverage

def main():
    print("=== CURSED Syntax Examples Generator ===\n")
    
    # Generate syntax examples
    print("1. Generating Syntax Examples:")
    print("-" * 40)
    save_examples()
    
    # Analyze coverage
    print("\n2. Grammar Coverage Analysis:")
    print("-" * 40)
    
    coverage = analyze_syntax_coverage()
    total_elements = len(coverage)
    fully_covered = 0
    
    for element, data in coverage.items():
        percentage = data['percentage']
        status = "✓" if percentage == 100 else "⚠" if percentage >= 50 else "✗"
        print(f"{status} {element:20} {data['covered']:2}/{data['total']:2} ({percentage:5.1f}%)")
        
        if percentage == 100:
            fully_covered += 1
    
    overall_percentage = (fully_covered / total_elements) * 100
    
    print(f"\nOverall Grammar Coverage: {fully_covered}/{total_elements} ({overall_percentage:.1f}%) elements fully covered")
    
    # Summary
    print("\n=== Test Coverage Summary ===")
    examples = create_syntax_examples()
    print(f"Generated {len(examples)} comprehensive test examples")
    print("Coverage includes:")
    for name in examples.keys():
        print(f"  • {name.replace('_', ' ').title()}")
    
    print("\n=== Usage Instructions ===")
    print("1. Run CURSED parser on each test file:")
    print("   ./cursed parse test_*.csd")
    print("2. Check for parser errors and AST generation")
    print("3. Verify grammar compliance and error recovery")
    print("4. Test with malformed input for robustness")
    
    if overall_percentage >= 90:
        print("\n✅ Excellent: Test examples provide comprehensive grammar coverage")
    elif overall_percentage >= 70:
        print("\n✓ Good: Test examples cover most grammar elements")
    else:
        print("\n⚠️ Fair: Consider adding more examples for better coverage")

if __name__ == "__main__":
    main()
