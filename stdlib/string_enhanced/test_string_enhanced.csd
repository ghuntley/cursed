yeet "testz"
yeet "string_enhanced"

fr fr ================================
fr fr String Enhanced Module Tests
fr fr Comprehensive testing of advanced string operations
fr fr ================================

test_start("String Intern Test")
sus intern StringIntern = StringIntern_new()
(id1, intern) := StringIntern_intern(intern, "hello")
(id2, intern) := StringIntern_intern(intern, "world")
(id3, intern) := StringIntern_intern(intern, "hello") fr fr Should return same ID
assert_eq_int(id1, id3)
assert_true(id1 != id2)
sus retrieved tea = StringIntern_get(intern, id1)
assert_eq_string(retrieved, "hello")

test_start("String Scanner Test")
sus scanner StringScanner = StringScanner_new("hello world")
sus current sip = StringScanner_current_char(scanner)
assert_true(current == 'h')
sus peek sip = StringScanner_peek_char(scanner, 1)
assert_true(peek == 'e')
scanner = StringScanner_advance(scanner)
assert_true(scanner.position == 1)
assert_true(scanner.column == 2)

test_start("String Scanner Skip Whitespace Test")
sus whitespace_scanner StringScanner = StringScanner_new("   hello")
whitespace_scanner = StringScanner_skip_whitespace(whitespace_scanner)
sus after_whitespace sip = StringScanner_current_char(whitespace_scanner)
assert_true(after_whitespace == 'h')

test_start("String Scanner Read While Test")
sus alpha_scanner StringScanner = StringScanner_new("hello123")
(result, alpha_scanner) := StringScanner_read_while(alpha_scanner, "alpha")
assert_eq_string(result, "hello")
sus remaining tea = StringScanner_remaining(alpha_scanner)
assert_eq_string(remaining, "123")

test_start("Character Classification Test")
assert_true(StringScanner_is_alpha('a'))
assert_true(StringScanner_is_alpha('Z'))
assert_false(StringScanner_is_alpha('5'))
assert_true(StringScanner_is_digit('7'))
assert_false(StringScanner_is_digit('x'))
assert_true(StringScanner_char_matches_predicate('_', "ident"))
assert_true(StringScanner_char_matches_predicate('a', "alnum"))

test_start("Function Signature Formatting Test")
sus params []tea = ["x normie", "y tea"]
sus signature tea = format_function_signature("test_func", params, "lit")
sus expected tea = "slay test_func(x normie, y tea) lit"
assert_eq_string(signature, expected)

test_start("Variable Declaration Formatting Test")
sus var_decl tea = format_variable_declaration("count", "normie", "42")
sus expected_var tea = "sus count normie = 42"
assert_eq_string(var_decl, expected_var)

test_start("Array Type Formatting Test")
sus array_type tea = format_array_type("normie")
assert_eq_string(array_type, "[normie]")

test_start("Function Call Formatting Test")
sus args []tea = ["42", "\"hello\""]
sus call tea = format_function_call("test", args)
sus expected_call tea = "test(42, \"hello\")"
assert_eq_string(call, expected_call)

test_start("Identifier Validation Test")
assert_true(is_valid_identifier("valid_name"))
assert_true(is_valid_identifier("_underscore"))
assert_true(is_valid_identifier("var123"))
assert_false(is_valid_identifier("123invalid"))
assert_false(is_valid_identifier(""))
assert_false(is_valid_identifier("invalid-dash"))

test_start("CURSED Keyword Detection Test")
assert_true(is_cursed_keyword("slay"))
assert_true(is_cursed_keyword("sus"))
assert_true(is_cursed_keyword("damn"))
assert_true(is_cursed_keyword("vibes"))
assert_true(is_cursed_keyword("bestie"))
assert_true(is_cursed_keyword("squad"))
assert_false(is_cursed_keyword("regular_name"))
assert_false(is_cursed_keyword("not_keyword"))

test_start("String Escape Test")
sus escaped tea = escape_string_literal("hello\nworld")
assert_true(string_contains(escaped, "\\n"))
assert_true(string_starts_with(escaped, "\""))
assert_true(string_ends_with(escaped, "\""))

test_start("String Unescape Test")
sus unescaped tea = unescape_string_literal("\"hello\\nworld\"")
assert_eq_string(unescaped, "hello\nworld")

test_start("Module Path Normalization Test")
sus normalized tea = normalize_module_path("some/../path")
assert_eq_string(normalized, "some/path")

test_start("Module Path Conversion Test")
sus file_path tea = module_path_to_file_path("module::submodule")
assert_eq_string(file_path, "module/submodule.csd")
sus module_path tea = file_path_to_module_path("module/submodule.csd")
assert_eq_string(module_path, "module::submodule")

test_start("Indentation Manager Test")
sus manager IndentationManager = IndentationManager_new("  ")
assert_eq_string(IndentationManager_current_indent(manager), "")
manager = IndentationManager_increase(manager)
assert_eq_string(IndentationManager_current_indent(manager), "  ")
manager = IndentationManager_increase(manager)
assert_eq_string(IndentationManager_current_indent(manager), "    ")
manager = IndentationManager_decrease(manager)
assert_eq_string(IndentationManager_current_indent(manager), "  ")

test_start("Indentation Line Test")
sus indented tea = IndentationManager_indent_line(manager, "code line")
assert_eq_string(indented, "  code line")

test_start("Case Conversion Test")
sus snake tea = to_snake_case("CamelCase")
assert_eq_string(snake, "camel_case")
sus pascal tea = to_pascal_case("snake_case")
assert_eq_string(pascal, "SnakeCase")

test_start("Scanner At End Test")
sus end_scanner StringScanner = StringScanner_new("abc")
assert_false(StringScanner_is_at_end(end_scanner))
end_scanner = StringScanner_advance(end_scanner)
end_scanner = StringScanner_advance(end_scanner)
end_scanner = StringScanner_advance(end_scanner)
assert_true(StringScanner_is_at_end(end_scanner))

print_test_summary()
