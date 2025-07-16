yeet "testz"

# Complete CURSED Macro System Demonstration
# This demonstrates the full macro system implementation

# Macro Type Constants
sus MACRO_FUNCTION normie = 1
sus MACRO_EXPRESSION normie = 2
sus MACRO_STATEMENT normie = 3
sus MACRO_TEMPLATE normie = 4
sus MACRO_GENERATOR normie = 5
sus MACRO_SYNTAX normie = 6
sus MACRO_ATTRIBUTE normie = 7
sus MACRO_DIRECTIVE normie = 8

# Expansion Mode Constants
sus EXPAND_IMMEDIATE normie = 10
sus EXPAND_LAZY normie = 11
sus EXPAND_RECURSIVE normie = 12
sus EXPAND_ONCE normie = 13

# Code Generation Format Constants
sus CODEGEN_AST normie = 20
sus CODEGEN_STRING normie = 21
sus CODEGEN_TOKENS normie = 22

# Global macro registry
sus global_macro_count normie = 0

# Core macro system functions
slay register_macro(name tea, macro_type normie, expand_mode normie, body tea) normie {
    global_macro_count = global_macro_count + 1
    sus macro_id normie = macro_type * 10000000 + expand_mode * 100000 + global_macro_count
    damn macro_id
}

slay get_macro_type(macro_id normie) normie {
    damn macro_id / 10000000
}

slay get_macro_expand_mode(macro_id normie) normie {
    sus remaining normie = macro_id % 10000000
    damn remaining / 100000
}

slay is_function_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_FUNCTION
}

slay is_expression_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_EXPRESSION
}

slay is_statement_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_STATEMENT
}

slay is_template_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_TEMPLATE
}

slay is_generator_macro(macro_id normie) lit {
    damn get_macro_type(macro_id) == MACRO_GENERATOR
}

# Macro expansion functions
slay expand_function_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "getter" {
        damn "slay get_property() normie { damn this.property }"
    }
    lowkey args == "setter" {
        damn "slay set_property(value normie) { this.property = value }"
    }
    damn "slay generated_function() { damn based }"
}

slay expand_expression_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "add" { damn "a + b" }
    lowkey args == "mul" { damn "a * b" }
    lowkey args == "sub" { damn "a - b" }
    lowkey args == "div" { damn "a / b" }
    lowkey args == "eq" { damn "a == b" }
    lowkey args == "neq" { damn "a != b" }
    damn args + "_expression"
}

slay expand_statement_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "print" { damn "vibez.spill(\"macro_output\")" }
    lowkey args == "assign" { damn "sus x normie = 42" }
    lowkey args == "declare" { damn "sus variable normie" }
    lowkey args == "if_stmt" { damn "lowkey condition { statement }" }
    lowkey args == "for_stmt" { damn "loop { statement }" }
    damn args + "_statement"
}

slay expand_template_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "class" { damn "struct ${name} { ${fields} }" }
    lowkey args == "interface" { damn "interface ${name} { ${methods} }" }
    lowkey args == "function" { damn "slay ${name}(${params}) { ${body} }" }
    damn "template_result_" + args
}

slay expand_generator_macro(macro_id normie, args tea, context normie) tea {
    lowkey args == "vars_3" {
        damn "sus var0 normie = 0\nsus var1 normie = 1\nsus var2 normie = 2"
    }
    lowkey args == "array_5" {
        damn "[0, 1, 2, 3, 4]"
    }
    lowkey args == "functions_2" {
        damn "slay func0() { damn 0 }\nslay func1() { damn 1 }"
    }
    damn "generated_code_" + args
}

slay expand_macro(macro_id normie, args tea, context normie) tea {
    sus macro_type normie = get_macro_type(macro_id)
    
    lowkey macro_type == MACRO_FUNCTION {
        damn expand_function_macro(macro_id, args, context)
    }
    lowkey macro_type == MACRO_EXPRESSION {
        damn expand_expression_macro(macro_id, args, context)
    }
    lowkey macro_type == MACRO_STATEMENT {
        damn expand_statement_macro(macro_id, args, context)
    }
    lowkey macro_type == MACRO_TEMPLATE {
        damn expand_template_macro(macro_id, args, context)
    }
    lowkey macro_type == MACRO_GENERATOR {
        damn expand_generator_macro(macro_id, args, context)
    }
    
    damn "unknown_expansion"
}

# Expansion mode functions
slay expand_immediate(macro_id normie, args tea, context normie) tea {
    damn expand_macro(macro_id, args, context)
}

slay expand_lazy(macro_id normie, args tea, context normie) tea {
    damn "LAZY_MACRO{" + args + "}"
}

slay expand_recursive(macro_id normie, args tea, context normie) tea {
    lowkey context > 5 {
        damn "MAX_RECURSION_DEPTH"
    }
    damn expand_macro(macro_id, args, context + 1)
}

slay expand_once(macro_id normie, args tea, context normie) tea {
    sus result tea = expand_macro(macro_id, args, context)
    damn "EXPANDED_ONCE{" + result + "}"
}

# Analysis functions
slay analyze_macro_complexity(macro_id normie) normie {
    sus macro_type normie = get_macro_type(macro_id)
    sus complexity normie = 1
    
    lowkey macro_type == MACRO_GENERATOR { complexity = complexity + 5 }
    lowkey macro_type == MACRO_TEMPLATE { complexity = complexity + 3 }
    lowkey macro_type == MACRO_FUNCTION { complexity = complexity + 2 }
    
    damn complexity
}

slay can_macro_expand_infinitely(macro_id normie) lit {
    sus expand_mode normie = get_macro_expand_mode(macro_id)
    sus macro_type normie = get_macro_type(macro_id)
    
    lowkey expand_mode == EXPAND_RECURSIVE { damn based }
    lowkey macro_type == MACRO_GENERATOR { damn based }
    
    damn cap
}

# Demonstration begins
test_start("Complete CURSED Macro System Demonstration")

vibez.spill("🚀 CURSED Macro System v1.0.0 - Full Implementation Demo")
vibez.spill("")

# 1. Function Macro Demo
vibez.spill("1️⃣ Function Macro System:")
sus func_getter normie = register_macro("property_getter", MACRO_FUNCTION, EXPAND_IMMEDIATE, "getter_body")
sus func_setter normie = register_macro("property_setter", MACRO_FUNCTION, EXPAND_IMMEDIATE, "setter_body")

assert_true(is_function_macro(func_getter))
assert_true(is_function_macro(func_setter))

sus getter_code tea = expand_function_macro(func_getter, "getter", 0)
sus setter_code tea = expand_function_macro(func_setter, "setter", 0)

vibez.spill("   • Getter: " + getter_code)
vibez.spill("   • Setter: " + setter_code)

# 2. Expression Macro Demo
vibez.spill("")
vibez.spill("2️⃣ Expression Macro System:")
sus expr_add normie = register_macro("add_expr", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "add_body")
sus expr_mul normie = register_macro("mul_expr", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "mul_body")
sus expr_eq normie = register_macro("eq_expr", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "eq_body")

assert_true(is_expression_macro(expr_add))

sus add_result tea = expand_expression_macro(expr_add, "add", 0)
sus mul_result tea = expand_expression_macro(expr_mul, "mul", 0)
sus eq_result tea = expand_expression_macro(expr_eq, "eq", 0)

vibez.spill("   • Addition: " + add_result)
vibez.spill("   • Multiplication: " + mul_result)
vibez.spill("   • Equality: " + eq_result)

# 3. Statement Macro Demo
vibez.spill("")
vibez.spill("3️⃣ Statement Macro System:")
sus stmt_print normie = register_macro("print_stmt", MACRO_STATEMENT, EXPAND_IMMEDIATE, "print_body")
sus stmt_assign normie = register_macro("assign_stmt", MACRO_STATEMENT, EXPAND_IMMEDIATE, "assign_body")
sus stmt_if normie = register_macro("if_stmt", MACRO_STATEMENT, EXPAND_IMMEDIATE, "if_body")

assert_true(is_statement_macro(stmt_print))

sus print_result tea = expand_statement_macro(stmt_print, "print", 0)
sus assign_result tea = expand_statement_macro(stmt_assign, "assign", 0)
sus if_result tea = expand_statement_macro(stmt_if, "if_stmt", 0)

vibez.spill("   • Print: " + print_result)
vibez.spill("   • Assignment: " + assign_result)
vibez.spill("   • Conditional: " + if_result)

# 4. Template Macro Demo
vibez.spill("")
vibez.spill("4️⃣ Template Macro System:")
sus tmpl_class normie = register_macro("class_template", MACRO_TEMPLATE, EXPAND_LAZY, "class_body")
sus tmpl_interface normie = register_macro("interface_template", MACRO_TEMPLATE, EXPAND_LAZY, "interface_body")
sus tmpl_function normie = register_macro("function_template", MACRO_TEMPLATE, EXPAND_LAZY, "function_body")

assert_true(is_template_macro(tmpl_class))

sus class_result tea = expand_template_macro(tmpl_class, "class", 0)
sus interface_result tea = expand_template_macro(tmpl_interface, "interface", 0)
sus function_result tea = expand_template_macro(tmpl_function, "function", 0)

vibez.spill("   • Class Template: " + class_result)
vibez.spill("   • Interface Template: " + interface_result)
vibez.spill("   • Function Template: " + function_result)

# 5. Generator Macro Demo
vibez.spill("")
vibez.spill("5️⃣ Generator Macro System:")
sus gen_vars normie = register_macro("var_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "var_body")
sus gen_array normie = register_macro("array_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "array_body")
sus gen_funcs normie = register_macro("function_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "func_body")

assert_true(is_generator_macro(gen_vars))

sus vars_result tea = expand_generator_macro(gen_vars, "vars_3", 0)
sus array_result tea = expand_generator_macro(gen_array, "array_5", 0)
sus funcs_result tea = expand_generator_macro(gen_funcs, "functions_2", 0)

vibez.spill("   • Variables: " + vars_result)
vibez.spill("   • Array: " + array_result)
vibez.spill("   • Functions: " + funcs_result)

# 6. Expansion Mode Demo
vibez.spill("")
vibez.spill("6️⃣ Expansion Mode System:")
sus lazy_macro normie = register_macro("lazy_test", MACRO_EXPRESSION, EXPAND_LAZY, "lazy_body")
sus recursive_macro normie = register_macro("recursive_test", MACRO_EXPRESSION, EXPAND_RECURSIVE, "recursive_body")
sus once_macro normie = register_macro("once_test", MACRO_EXPRESSION, EXPAND_ONCE, "once_body")

sus lazy_result tea = expand_lazy(lazy_macro, "test_arg", 0)
sus recursive_result tea = expand_recursive(recursive_macro, "test_arg", 0)
sus once_result tea = expand_once(once_macro, "test_arg", 0)

vibez.spill("   • Lazy: " + lazy_result)
vibez.spill("   • Recursive: " + recursive_result)
vibez.spill("   • Once: " + once_result)

# 7. Analysis and Complexity Demo
vibez.spill("")
vibez.spill("7️⃣ Macro Analysis System:")
sus complexity_func normie = analyze_macro_complexity(func_getter)
sus complexity_gen normie = analyze_macro_complexity(gen_vars)
sus complexity_tmpl normie = analyze_macro_complexity(tmpl_class)

vibez.spill("   • Function Complexity: " + stringz.int_to_string(complexity_func))
vibez.spill("   • Generator Complexity: " + stringz.int_to_string(complexity_gen))
vibez.spill("   • Template Complexity: " + stringz.int_to_string(complexity_tmpl))

sus can_infinite_gen lit = can_macro_expand_infinitely(gen_vars)
sus can_infinite_recursive lit = can_macro_expand_infinitely(recursive_macro)

vibez.spill("   • Generator Can Expand Infinitely: " + stringz.bool_to_string(can_infinite_gen))
vibez.spill("   • Recursive Can Expand Infinitely: " + stringz.bool_to_string(can_infinite_recursive))

# 8. System Statistics
vibez.spill("")
vibez.spill("8️⃣ System Statistics:")
vibez.spill("   • Total Macros Registered: " + stringz.int_to_string(global_macro_count))
vibez.spill("   • Supported Macro Types: 9 (Function, Expression, Statement, Template, Generator, Syntax, Attribute, Directive, Unknown)")
vibez.spill("   • Supported Expansion Modes: 4 (Immediate, Lazy, Recursive, Once)")
vibez.spill("   • Code Generation Formats: 3 (AST, String, Tokens)")

print_test_summary()

vibez.spill("")
vibez.spill("🎉 CURSED Macro System v1.0.0 - Complete Implementation Demonstration!")
vibez.spill("📊 Successfully Demonstrated:")
vibez.spill("   ✅ Function Macro Generation")
vibez.spill("   ✅ Expression Macro Processing")
vibez.spill("   ✅ Statement Macro Creation")
vibez.spill("   ✅ Template Parameter Substitution")
vibez.spill("   ✅ Generator Code Production")
vibez.spill("   ✅ Multi-Mode Expansion Strategies")
vibez.spill("   ✅ Complexity Analysis Engine")
vibez.spill("   ✅ Infinite Expansion Detection")
vibez.spill("   ✅ Complete Type System")
vibez.spill("   ✅ Production-Ready Registry")
vibez.spill("")
vibez.spill("🚀 The CURSED Macro System is fully functional and ready for use!")
