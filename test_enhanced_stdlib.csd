yeet "testz"
yeet "runtime_core"
yeet "string_enhanced"
yeet "hash_map_enhanced"

fr fr Comprehensive test suite for enhanced stdlib modules

test_start("Enhanced Runtime Core - Dynamic Arrays")
sus vec RuntimeVec<tea> = RuntimeVec_new<tea>()
vec = RuntimeVec_push(vec, "hello")
vec = RuntimeVec_push(vec, "world")
assert_eq_int(RuntimeVec_len(vec), 2)
assert_eq_string(RuntimeVec_get(vec, 0), "hello")
assert_eq_string(RuntimeVec_get(vec, 1), "world")

test_start("Enhanced Runtime Core - String Builder")
sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
sb = RuntimeStringBuilder_append(sb, "Hello")
sb = RuntimeStringBuilder_append(sb, " ")
sb = RuntimeStringBuilder_append(sb, "CURSED")
sb = RuntimeStringBuilder_append_char(sb, '!')
sus result tea = RuntimeStringBuilder_to_string(sb)
assert_eq_string(result, "Hello CURSED!")

test_start("Enhanced Runtime Core - Stack Operations")
sus stack RuntimeStack<normie> = RuntimeStack_new<normie>()
stack = RuntimeStack_push(stack, 10)
stack = RuntimeStack_push(stack, 20)
stack = RuntimeStack_push(stack, 30)

(top_val, peek_success) := RuntimeStack_peek(stack)
assert_true(peek_success)
assert_eq_int(top_val, 30)
assert_eq_int(RuntimeStack_size(stack), 3)

(popped_val, pop_success) := RuntimeStack_pop(stack)
assert_true(pop_success)
assert_eq_int(popped_val, 30)
assert_eq_int(RuntimeStack_size(stack), 2)

test_start("Enhanced HashMap - Symbol Table")
sus symbol_table SymbolTable<normie> = SymbolTable_new<normie>()
symbol_table = SymbolTable_insert(symbol_table, "variable1", 42)
symbol_table = SymbolTable_insert(symbol_table, "variable2", 100)

assert_eq_int(SymbolTable_size(symbol_table), 2)
assert_true(SymbolTable_contains(symbol_table, "variable1"))
assert_true(SymbolTable_contains(symbol_table, "variable2"))
assert_false(SymbolTable_contains(symbol_table, "nonexistent"))

(value1, found1) := SymbolTable_get(symbol_table, "variable1")
assert_true(found1)
assert_eq_int(value1, 42)

(value2, found2) := SymbolTable_get(symbol_table, "variable2")
assert_true(found2)
assert_eq_int(value2, 100)

test_start("Enhanced HashMap - Function Table")
sus func_table FunctionTable = FunctionTable_new()
sus func_info FunctionInfo = FunctionInfo{
    name: "test_function",
    return_type: "normie",
    parameter_types: []tea{"tea", "normie"},
    parameter_names: []tea{"name", "count"},
    is_generic: cringe,
    is_extern: cringe,
    definition_line: 42
}

func_table = FunctionTable_declare_function(func_table, func_info)
assert_true(FunctionTable_is_function_declared(func_table, "test_function"))

(retrieved_info, found_func) := FunctionTable_lookup_function(func_table, "test_function")
assert_true(found_func)
assert_eq_string(retrieved_info.name, "test_function")
assert_eq_string(retrieved_info.return_type, "normie")
assert_eq_int(retrieved_info.definition_line, 42)

test_start("Enhanced String - String Interning")
sus intern StringIntern = StringIntern_new()
(id1, intern1) := StringIntern_intern(intern, "identifier1")
(id2, intern2) := StringIntern_intern(intern1, "identifier2")
(id3, intern3) := StringIntern_intern(intern2, "identifier1")  fr fr Duplicate

assert_eq_int(id1, 0)
assert_eq_int(id2, 1)
assert_eq_int(id3, 0)  fr fr Should return same ID for duplicate

assert_eq_string(StringIntern_get(intern3, id1), "identifier1")
assert_eq_string(StringIntern_get(intern3, id2), "identifier2")

test_start("Enhanced String - Scanner Operations")
sus scanner StringScanner = StringScanner_new("hello_world123")
assert_eq_char(StringScanner_current_char(scanner), 'h')
assert_eq_char(StringScanner_peek_char(scanner, 1), 'e')

scanner = StringScanner_advance(scanner)
assert_eq_char(StringScanner_current_char(scanner), 'e')

(identifier, final_scanner) := StringScanner_read_while(scanner, "ident")
assert_eq_string(identifier, "ello_world123")

test_start("Enhanced String - Identifier Validation")
assert_true(is_valid_identifier("valid_name"))
assert_true(is_valid_identifier("_private"))
assert_true(is_valid_identifier("name123"))
assert_false(is_valid_identifier("123invalid"))
assert_false(is_valid_identifier(""))
assert_false(is_valid_identifier("invalid-name"))

test_start("Enhanced String - Keyword Detection")
assert_true(is_cursed_keyword("slay"))
assert_true(is_cursed_keyword("sus"))
assert_true(is_cursed_keyword("damn"))
assert_true(is_cursed_keyword("vibes"))
assert_false(is_cursed_keyword("regular_word"))

test_start("Enhanced String - Code Formatting")
sus params []tea = []tea{"param1 tea", "param2 normie"}
sus signature tea = format_function_signature("test_func", params, "lit")
assert_eq_string(signature, "slay test_func(param1 tea, param2 normie) lit")

sus var_decl tea = format_variable_declaration("count", "normie", "42")
assert_eq_string(var_decl, "sus count normie = 42")

test_start("Enhanced String - String Escaping")
sus escaped tea = escape_string_literal("hello \"world\" \n")
assert_eq_string(escaped, "\"hello \\\"world\\\" \\n\"")

sus unescaped tea = unescape_string_literal("\"hello \\\"world\\\" \\n\"")
assert_eq_string(unescaped, "hello \"world\" \n")

test_start("Enhanced String - Case Conversion")
sus snake_case tea = to_snake_case("CamelCaseString")
assert_eq_string(snake_case, "_camel_case_string")

sus pascal_case tea = to_pascal_case("snake_case_string")
assert_eq_string(pascal_case, "SnakeCaseString")

print_test_summary()
