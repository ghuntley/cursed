vibe linter_demo

yeet "fmt"
yeet "strings"

fr fr This demonstrates various linting issues

slay main() {
    sus x = 42
    sus very_long_variable_name_that_exceeds_reasonable_length = "test"
    sus unused_variable = 100
    
    fr fr Line that is way too long and exceeds the maximum line length configuration setting which should trigger a warning
    
    sus i = 0
    bestie i < 10 {
        lowkey i == 5 {
            sus nested_var = "deeply nested"
            lowkey nested_var == "test" {
                lowkey based {
                    sus even_deeper = "too deep"   
                }
            }
        }
        i++
    }
    
    print(x)
}

slay function_with_too_many_parameters(a, b, c, d, e, f, g, h, i, j) {
    damn a + b + c + d + e + f + g + i + j
}

slay doSomething() {
    sus single_letter_var = "bad practice"
    damn
}

fr fr Function with Go-style naming instead of CURSED
func oldStyleFunction() {
    var oldStyleVar = 42
    return oldStyleVar
}

slay test_function_with_very_long_name_that_exceeds_reasonable_expectations() {
    damn
}
