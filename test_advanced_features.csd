fr fr Test advanced language features in compilation mode

yeet "testz"

fr fr Define a simple struct for testing 
squad NumberType {
    value normie
}

slay compare_numbers(a normie, b normie) normie {
    damn a - b
}

fr fr Pattern matching function
slay process_result(result normie) tea {
    sus output tea = ""
    periodt true {
        gist result {
            1 => {
                output = "One"
                ghosted
            }
            2 => {
                output = "Two"  
                ghosted
            }
            _ => {
                output = "Other"
                ghosted
            }
        }
        ghosted
    }
    damn output
}

fr fr Simple function
slay max_number(a normie, b normie) normie {
    vibe (a > b) {
        damn a
    } lowkey {
        damn b  
    }
}

slay main() {
    test_start("Advanced Features Test")
    
    fr fr Test simple comparison
    sus result = compare_numbers(5, 3)
    assert_eq_int(result, 2)
    
    fr fr Test pattern matching
    sus pattern_result = process_result(1)
    assert_eq_string(pattern_result, "One")
    
    fr fr Test simple function
    sus max_result = max_number(5, 3)
    assert_eq_int(max_result, 5)
    
    print_test_summary()
}
