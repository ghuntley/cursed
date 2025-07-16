yeet "testz"

# Simple macro system test

# Macro Type Constants
sus MACRO_FUNCTION normie = 1
sus MACRO_EXPRESSION normie = 2
sus EXPAND_IMMEDIATE normie = 10

# Simple macro registry simulation
sus global_macro_count normie = 0

# Simple macro registration
slay register_macro(name tea, macro_type normie, expand_mode normie, body tea) normie {
    global_macro_count = global_macro_count + 1
    sus macro_id normie = macro_type * 1000 + expand_mode * 10 + global_macro_count
    damn macro_id
}

# Extract macro type
slay get_macro_type(macro_id normie) normie {
    damn macro_id / 1000
}

# Type checking
slay is_function_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_FUNCTION
}

slay is_expression_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_EXPRESSION
}

# Simple expansion
slay expand_expression_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "add" { damn "a + b" }
    lowkey args == "mul" { damn "a * b" }
    damn args + "_expression"
}

slay expand_macro(macro_id normie, args tea, context normie) tea {
    sus macro_type normie = get_macro_type(macro_id)
    lowkey macro_type == MACRO_EXPRESSION {
        damn expand_expression_macro(macro_id, args, context)
    }
    damn "unknown_expansion"
}

# Test the basic functionality
test_start("Simple macro system test")

sus test_macro normie = register_macro("test", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "body")
assert_true(test_macro != 0)

assert_true(is_expression_macro(test_macro))
assert_true(!is_function_macro(test_macro))

sus result tea = expand_expression_macro(test_macro, "add", 0)
assert_true(result == "a + b")

sus general_result tea = expand_macro(test_macro, "mul", 0)
assert_true(general_result == "a * b")

print_test_summary()

vibez.spill("✅ Simple macro system test passed!")
vibez.spill("📊 Key Features Tested:")
vibez.spill("   • Macro registration")
vibez.spill("   • Type checking")
vibez.spill("   • Basic expansion")
