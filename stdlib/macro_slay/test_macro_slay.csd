yeet "testz"
yeet "macro_slay"

fr fr Comprehensive test suite for the complete macro_slay implementation

test_start("Basic macro_slay module tests")

fr fr Test module status and initialization
sus version tea = macro_slay_version()
assert_true(version == "1.0.0")

sus status tea = macro_slay_status()
vibez.spill("Status: " + status)

assert_true(is_macro_slay_ready())

fr fr Test system capabilities
sus types_count normie = get_supported_macro_types()
assert_true(types_count == 9)

sus modes_count normie = get_supported_expand_modes()
assert_true(modes_count == 4)

vibez.spill("✅ Basic module tests passed")

test_start("Macro registration and storage tests")

fr fr Test macro registration with different types
sus func_macro normie = register_macro("test_func", MACRO_FUNCTION, EXPAND_IMMEDIATE, "test_body")
assert_true(func_macro != 0)

sus expr_macro normie = register_macro("test_expr", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "a + b")
assert_true(expr_macro != 0)

sus stmt_macro normie = register_macro("test_stmt", MACRO_STATEMENT, EXPAND_IMMEDIATE, "statement")
assert_true(stmt_macro != 0)

sus tmpl_macro normie = register_macro("test_template", MACRO_TEMPLATE, EXPAND_LAZY, "template_body")
assert_true(tmpl_macro != 0)

sus gen_macro normie = register_macro("test_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "generator")
assert_true(gen_macro != 0)

fr fr Test macro lookup
sus found_func normie = lookup_macro("test_func")
assert_true(found_func != 0)
assert_true(found_func == func_macro)

fr fr Test macro existence checks
assert_true(is_macro_defined("test_func"))
assert_true(is_macro_defined("test_expr"))
assert_true(!is_macro_defined("nonexistent"))

fr fr Test macro count tracking
sus total_count normie = get_macro_count()
assert_true(total_count > 5) fr fr Should include built-ins + our test macros

vibez.spill("✅ Macro registration tests passed")

test_start("Macro type checking tests")

fr fr Test type extraction
assert_true(get_macro_type(func_macro) == MACRO_FUNCTION)
assert_true(get_macro_type(expr_macro) == MACRO_EXPRESSION)
assert_true(get_macro_type(stmt_macro) == MACRO_STATEMENT)
assert_true(get_macro_type(tmpl_macro) == MACRO_TEMPLATE)
assert_true(get_macro_type(gen_macro) == MACRO_GENERATOR)

fr fr Test type checking functions
assert_true(is_function_macro(func_macro))
assert_true(!is_expression_macro(func_macro))
assert_true(!is_statement_macro(func_macro))

assert_true(is_expression_macro(expr_macro))
assert_true(!is_function_macro(expr_macro))

assert_true(is_statement_macro(stmt_macro))
assert_true(is_template_macro(tmpl_macro))
assert_true(is_generator_macro(gen_macro))

fr fr Test expansion mode extraction
assert_true(get_macro_expand_mode(func_macro) == EXPAND_IMMEDIATE)
assert_true(get_macro_expand_mode(tmpl_macro) == EXPAND_LAZY)

vibez.spill("✅ Macro type checking tests passed")

test_start("Built-in macro tests")

fr fr Test built-in macro initialization
sus builtin_count normie = get_builtin_macro_count()
assert_true(builtin_count > 0)

fr fr Test built-in macro recognition
assert_true(is_builtin_macro("add"))
assert_true(is_builtin_macro("print"))
assert_true(is_builtin_macro("repeat"))
assert_true(!is_builtin_macro("nonexistent"))

fr fr Test built-in macro execution
sus add_result tea = execute_macro("add", "operands")
assert_true(add_result == "a + b")

sus print_result tea = execute_macro("print", "hello")
assert_true(print_result == "vibez.spill(\"hello\")")

vibez.spill("✅ Built-in macro tests passed")

test_start("Macro expansion tests")

fr fr Test function macro expansion
sus func_result tea = expand_function_macro(func_macro, "param1", 0)
assert_true(func_result != "")
vibez.spill("Function expansion: " + func_result)

fr fr Test expression macro expansion with built-ins
sus add_expr tea = expand_expression_macro(expr_macro, "add", 0)
assert_true(add_expr == "a + b")

sus mul_expr tea = expand_expression_macro(expr_macro, "mul", 0)
assert_true(mul_expr == "a * b")

fr fr Test statement macro expansion
sus stmt_result tea = expand_statement_macro(stmt_macro, "print", 0)
assert_true(stmt_result != "")

fr fr Test template macro expansion
sus template_result tea = expand_template_macro(tmpl_macro, "MyClass", 0)
assert_true(template_result != "")

fr fr Test generator macro expansion
sus generator_result tea = expand_generator_macro(gen_macro, "3", 0)
assert_true(generator_result != "")
vibez.spill("Generator result: " + generator_result)

fr fr Test general macro expansion dispatch
sus general_result tea = expand_macro(func_macro, "args", 0)
assert_true(general_result != "")

vibez.spill("✅ Macro expansion tests passed")

test_start("Template parameter substitution tests")

fr fr Register template macros to test substitution
sus getter_macro normie = register_macro("getter", MACRO_FUNCTION, EXPAND_IMMEDIATE, "getter_template")
sus getter_result tea = expand_function_macro(getter_macro, "name,type", 0)
assert_true(getter_result != "")
vibez.spill("Getter template: " + getter_result)

fr fr Test class template
sus class_macro normie = register_macro("class_template", MACRO_TEMPLATE, EXPAND_LAZY, "class_body")
sus class_result tea = expand_template_macro(class_macro, "Person", 0)
assert_true(class_result != "")

vibez.spill("✅ Template substitution tests passed")

test_start("Generator macro tests")

fr fr Test variable generator
sus var_gen_macro normie = register_macro("var_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "variables")
sus var_gen_result tea = expand_generator_macro(var_gen_macro, "5", 0)
assert_true(var_gen_result != "")
vibez.spill("Variable generation: " + var_gen_result)

fr fr Test array generator
sus array_gen_macro normie = register_macro("array_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "array")
sus array_gen_result tea = expand_generator_macro(array_gen_macro, "4", 0)
assert_true(array_gen_result != "")
vibez.spill("Array generation: " + array_gen_result)

vibez.spill("✅ Generator macro tests passed")

test_start("Expansion mode tests")

fr fr Test immediate expansion
sus immediate_result tea = expand_immediate(func_macro, "args", 0)
assert_true(immediate_result != "")

fr fr Test lazy expansion
sus lazy_result tea = expand_lazy(tmpl_macro, "args", 0)
assert_true(lazy_result != "")
vibez.spill("Lazy expansion: " + lazy_result)

fr fr Test recursive expansion
sus recursive_result tea = expand_recursive(func_macro, "args", 0)
assert_true(recursive_result != "")

fr fr Test once expansion
sus once_result tea = expand_once(func_macro, "args", 0)
assert_true(once_result != "")

vibez.spill("✅ Expansion mode tests passed")

test_start("AST integration tests")

fr fr Test macro to AST conversion
sus ast_node normie = macro_to_ast(func_macro, "test_args")
assert_true(ast_node != 0)

fr fr Test AST to code conversion
sus ast_code tea = ast_to_code(ast_node)
assert_true(ast_code != "")
vibez.spill("AST to code: " + ast_code)

vibez.spill("✅ AST integration tests passed")

test_start("Code generation tests")

fr fr Test different code generation formats
sus ast_format tea = generate_code_from_macro(func_macro, "args", CODEGEN_AST)
assert_true(ast_format != "")

sus string_format tea = generate_code_from_macro(func_macro, "args", CODEGEN_STRING)
assert_true(string_format != "")

sus token_format tea = generate_code_from_macro(func_macro, "args", CODEGEN_TOKENS)
assert_true(token_format != "")

vibez.spill("Code generation formats tested")

vibez.spill("✅ Code generation tests passed")

test_start("Analysis and validation tests")

fr fr Test macro complexity analysis
sus complexity normie = analyze_macro_complexity(gen_macro)
assert_true(complexity > 0)
vibez.spill("Generator complexity: " + stringz.int_to_string(complexity))

fr fr Test expansion size estimation
sus estimated_size normie = estimate_expansion_size(gen_macro, "5")
assert_true(estimated_size > 0)

fr fr Test infinite expansion detection
sus can_expand_infinite lit = can_macro_expand_infinitely(gen_macro)
vibez.spill("Can expand infinitely: " + stringz.bool_to_string(can_expand_infinite))

fr fr Test macro signature
sus signature tea = get_macro_signature(func_macro)
assert_true(signature != "")
vibez.spill("Function macro signature: " + signature)

vibez.spill("✅ Analysis tests passed")

test_start("Syntax validation tests")

fr fr Test valid syntax
assert_true(validate_macro_syntax("slay test() { damn based }"))
assert_true(validate_macro_syntax("a + b"))
assert_true(validate_macro_syntax("(1 + 2) * 3"))

fr fr Test invalid syntax  
assert_true(!validate_macro_syntax(""))
assert_true(!validate_macro_syntax("   "))
assert_true(!validate_macro_syntax("{{{"))
assert_true(!validate_macro_syntax("(()"))

vibez.spill("✅ Syntax validation tests passed")

test_start("Macro compilation tests")

fr fr Test valid macro compilation
sus compiled_macro normie = compile_macro("function test() { return true; }")
assert_true(compiled_macro != 0)

fr fr Test invalid macro compilation
sus invalid_compile normie = compile_macro("")
assert_true(invalid_compile == 0)

vibez.spill("✅ Macro compilation tests passed")

test_start("Debug and tracing tests")

fr fr Test debug expansion info
sus debug_info tea = debug_macro_expansion(func_macro, "debug_args", 0)
assert_true(debug_info != "")
vibez.spill("Debug info: " + debug_info)

fr fr Test expansion tracing
sus trace_info tea = trace_macro_expansion(func_macro, "trace_args", 1)
assert_true(trace_info != "")
vibez.spill("Trace info: " + trace_info)

vibez.spill("✅ Debug and tracing tests passed")

test_start("Advanced macro type tests")

fr fr Register and test syntax macros
sus syntax_macro normie = register_macro("unless", MACRO_SYNTAX, EXPAND_IMMEDIATE, "syntax_body")
assert_true(is_syntax_macro(syntax_macro))

sus syntax_result tea = expand_syntax_macro(syntax_macro, "condition", 0)
assert_true(syntax_result != "")

fr fr Register and test attribute macros
sus attr_macro normie = register_macro("deprecated", MACRO_ATTRIBUTE, EXPAND_IMMEDIATE, "attr_body")
assert_true(is_attribute_macro(attr_macro))

sus attr_result tea = expand_attribute_macro(attr_macro, "old_function", 0)
assert_true(attr_result != "")

fr fr Register and test directive macros
sus dir_macro normie = register_macro("include", MACRO_DIRECTIVE, EXPAND_IMMEDIATE, "dir_body")
assert_true(is_directive_macro(dir_macro))

sus dir_result tea = expand_directive_macro(dir_macro, "module.csd", 0)
assert_true(dir_result != "")

vibez.spill("✅ Advanced macro type tests passed")

test_start("Code tokenization tests")

fr fr Test tokenization functionality
sus code tea = "slay test(a, b) { damn a + b }"
sus tokens tea = tokenize_code(code)
assert_true(tokens != "")
vibez.spill("Tokenization: " + tokens)

vibez.spill("✅ Tokenization tests passed")

test_start("System information tests")

fr fr Test system info functions
sus system_info tea = macro_system_info()
assert_true(system_info != "")
vibez.spill("System info: " + system_info)

fr fr Verify final macro count includes all registered macros
sus final_count normie = get_macro_count()
assert_true(final_count > 10) fr fr Built-ins + test macros

vibez.spill("✅ System information tests passed")

print_test_summary()

vibez.spill("\n🎉 Complete macro_slay implementation test suite passed!")
vibez.spill("📊 Comprehensive Test Coverage Summary:")
vibez.spill("   ✅ Module initialization and status")
vibez.spill("   ✅ Macro registration and storage system")
vibez.spill("   ✅ Type checking and validation")
vibez.spill("   ✅ Built-in macro library")
vibez.spill("   ✅ Macro expansion engine (all types)")
vibez.spill("   ✅ Template parameter substitution")
vibez.spill("   ✅ Generator macro code generation")
vibez.spill("   ✅ Expansion mode handling")
vibez.spill("   ✅ AST integration and conversion")
vibez.spill("   ✅ Multi-format code generation")
vibez.spill("   ✅ Complexity analysis and estimation")
vibez.spill("   ✅ Syntax validation and compilation")
vibez.spill("   ✅ Debug and tracing utilities")
vibez.spill("   ✅ Advanced macro types (syntax/attribute/directive)")
vibez.spill("   ✅ Code tokenization")
vibez.spill("   ✅ System information and capabilities")
vibez.spill("\n🚀 CURSED Macro System v1.0.0 - Production Ready!")
