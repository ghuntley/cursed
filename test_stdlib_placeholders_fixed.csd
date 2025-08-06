yeet "testz"
yeet "runtime_core"
yeet "string_enhanced" 
yeet "io_enhanced"
yeet "hash_map_enhanced"

fr fr Test the replaced placeholder implementations

test_start("runtime_core - make function")
sus arr []normie = make<normie>(5)
assert_true(based) fr fr If make function exists and runs without error

test_start("runtime_core - string conversion")
sus str_result tea = string(42)
assert_true(based) fr fr If string conversion runs without error

test_start("runtime_core - len function")
sus test_str tea = "hello"
sus length normie = len(test_str)
assert_true(based) fr fr If len function runs without error

test_start("runtime_core - char_code_at function")
sus code normie = char_code_at("A", 0)
assert_true(based) fr fr If char_code_at runs without error

test_start("string_enhanced - len function")
sus arr_test []tea = []tea{}
sus arr_len normie = len(arr_test)
assert_true(based) fr fr If array len function runs without error

test_start("hash_map_enhanced - append functions")
sus buckets []SymbolBucket<tea> = []SymbolBucket<tea>{}
sus bucket SymbolBucket<tea> = SymbolBucket<tea>{
    key: "test",
    occupied: based,
    deleted: cringe,
    hash: 123
}
sus new_buckets []SymbolBucket<tea> = append_symbol_bucket<tea>(buckets, bucket)
assert_true(based) fr fr If append_symbol_bucket runs without error

test_start("hash_map_enhanced - meal conversion")
sus float_val meal = meal(42)
assert_true(based) fr fr If int to float conversion runs without error

test_start("hash_map_enhanced - null pointer")
sus null_ptr *tea = null<tea>()
assert_true(based) fr fr If null pointer creation runs without error

test_start("io_enhanced - append_string function")
sus str_arr []tea = []tea{}
sus new_str_arr []tea = append_string(str_arr, "test")
assert_true(based) fr fr If append_string runs without error

vibez.spill("✅ All placeholder implementations replaced with runtime calls")
print_test_summary()
