// Comprehensive test suite for CURSED linter
// Tests all major categories of lint rules

yeet "testz"
yeet "stringz" 
yeet "arrayz"

// Test file with various issues for linter validation

// --- SECURITY VULNERABILITIES --- //

// Hardcoded secrets (should trigger critical)
sus api_key tea = "sk_live_1234567890abcdef"
sus password tea = "admin123"
sus secret_token tea = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

// SQL injection vulnerability (should trigger critical)
slay unsafe_query(user_id tea) tea {
    sus query tea = "SELECT * FROM users WHERE id=" + user_id
    damn query
}

// Command injection vulnerability (should trigger critical)
slay execute_command(user_input tea) {
    exec("ls " + user_input)
}

// Weak cryptography (should trigger warning)
slay weak_hash(data tea) tea {
    damn md5(data)
}

// --- PERFORMANCE ISSUES --- //

// String concatenation in loop (should trigger info)
slay build_string() tea {
    sus result tea = ""
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        result = result + "item" + int_to_str(i)  // Performance issue
    }
    damn result
}

// Inefficient array length in loop (should trigger hint)
slay process_array(arr []drip) {
    bestie (sus i drip = 0; i < len(arr); i = i + 1) {  // Should cache len(arr)
        vibez.spill(int_to_str(arr[i]))
    }
}

// Memory allocation in loop (should trigger warning)
slay allocate_in_loop() {
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        sus temp []drip = []  // Allocation in loop
        push(temp, i)
    }
}

// Nested loops (should trigger info)
slay nested_processing(matrix [][]drip) {
    bestie (sus i drip = 0; i < len(matrix); i = i + 1) {
        bestie (sus j drip = 0; j < len(matrix[i]); j = j + 1) {  // Nested loops
            matrix[i][j] = matrix[i][j] * 2
        }
    }
}

// --- CODE QUALITY ISSUES --- //

// Function too long (should trigger warning)
slay very_long_function() {
    vibez.spill("Line 1")
    vibez.spill("Line 2")
    vibez.spill("Line 3")
    vibez.spill("Line 4")
    vibez.spill("Line 5")
    vibez.spill("Line 6")
    vibez.spill("Line 7")
    vibez.spill("Line 8")
    vibez.spill("Line 9")
    vibez.spill("Line 10")
    vibez.spill("Line 11")
    vibez.spill("Line 12")
    vibez.spill("Line 13")
    vibez.spill("Line 14")
    vibez.spill("Line 15")
    vibez.spill("Line 16")
    vibez.spill("Line 17")
    vibez.spill("Line 18")
    vibez.spill("Line 19")
    vibez.spill("Line 20")
    vibez.spill("Line 21")
    vibez.spill("Line 22")
    vibez.spill("Line 23")
    vibez.spill("Line 24")
    vibez.spill("Line 25")
    vibez.spill("Line 26")
    vibez.spill("Line 27")
    vibez.spill("Line 28")
    vibez.spill("Line 29")
    vibez.spill("Line 30")
    vibez.spill("Line 31")
    vibez.spill("Line 32")
    vibez.spill("Line 33")
    vibez.spill("Line 34")
    vibez.spill("Line 35")
    vibez.spill("Line 36")
    vibez.spill("Line 37")
    vibez.spill("Line 38")
    vibez.spill("Line 39")
    vibez.spill("Line 40")
    vibez.spill("Line 41")
    vibez.spill("Line 42")
    vibez.spill("Line 43")
    vibez.spill("Line 44")
    vibez.spill("Line 45")
    vibez.spill("Line 46")
    vibez.spill("Line 47")
    vibez.spill("Line 48")
    vibez.spill("Line 49")
    vibez.spill("Line 50")
    vibez.spill("Line 51")
    vibez.spill("Line 52")  // Exceeds 50 line limit
}

// Too many parameters (should trigger warning)  
slay too_many_params(a drip, b drip, c drip, d drip, e drip, f drip, g drip) {
    damn a + b + c + d + e + f + g
}

// Excessive nesting (should trigger warning)
slay deeply_nested() {
    ready (based) {                    // Nesting level 1
        ready (based) {                // Nesting level 2
            ready (based) {            // Nesting level 3
                ready (based) {        // Nesting level 4
                    ready (based) {    // Nesting level 5 - exceeds limit
                        vibez.spill("Too deep!")
                    }
                }
            }
        }
    }
}

// Magic numbers (should trigger hints)
slay calculate_area(radius drip) drip {
    damn 3.14159 * radius * radius  // Magic number
}

slay process_data() {
    sus buffer_size drip = 8192    // Magic number
    sus timeout drip = 30000       // Magic number
}

// Line too long (should trigger warning)
sus very_long_line tea = "This is an extremely long line that exceeds the maximum line length configured for the linter and should trigger a warning about line length"

// Trailing whitespace (lines below have intentional trailing spaces)
sus trailing_space tea = "test"    
sus another_line tea = "data"	

// --- CURSED STYLE ISSUES --- //

// Non-Gen Z syntax (should trigger hints)
sus boolean_true lit = true     // Should use "based"
sus boolean_false lit = false   // Should use "cringe"

// Non-CURSED output (should trigger hint)
slay old_style_output() {
    print("Hello world")            // Should use vibez.spill
    println("Another message")      // Should use vibez.spill
}

// Non-CURSED imports (should trigger hint)
import "oldmodule"                  // Should use yeet

// Non-CURSED functions (should trigger hint) 
function old_function() {           // Should use slay
    damn "old style"
}

// Non-CURSED structs (should trigger hint)
struct OldStruct {                  // Should use squad
    spill value drip
}

// --- UNUSED ITEMS --- //

// Unused variable (should trigger warning)
sus unused_variable drip = 42

// Unused function (should trigger warning)
slay never_called_function() {
    damn "never used"
}

// Used variable (should not trigger warning)
sus used_variable tea = "hello"

// Used function (should not trigger warning)
slay called_function() tea {
    damn "I am used"
}

// --- GOOD EXAMPLES --- //

// Proper CURSED style
slay proper_cursed_function(param drip) tea {
    sus result tea = "Processing: " + int_to_str(param)
    vibez.spill(result)
    damn result
}

// Proper error handling
slay safe_operation() yikes<tea> {
    ready (some_condition()) {
        yikes "Operation failed"
    }
    damn "Success"
}

// Proper module usage
yeet "stringz"
yeet "mathz"

// Main function to test the linter
slay main() {
    // Use the variables to avoid unused warnings
    vibez.spill(used_variable)
    vibez.spill(called_function())
    vibez.spill(proper_cursed_function(123))
    
    // Call safe_operation with error handling
    sus result tea = safe_operation() fam {
        when _ -> damn "Error occurred"
    }
    vibez.spill(result)
}

// Helper function for testing
slay some_condition() lit {
    damn based
}
