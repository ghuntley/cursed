yeet "testz"
yeet "collections_core"
yeet "runtime_core" 
yeet "io_simple"

# Test Runtime Implementations
test_start("Runtime Implementations Test")

# Test Collections Core (FFI-free)
vibez.spill("Testing Collections Core...")
sus vec *Vector = vector_new()
vector_push(vec, 42)
vector_push(vec, 84)
sus first_val normie = vector_get(vec, 0)
assert_eq_int(first_val, 42)
vector_free(vec)

# Test Runtime Core Memory Management
vibez.spill("Testing Runtime Core...")
sus addr normie = runtime_allocate_memory(1024)
assert_true(addr > 0)
runtime_deallocate_memory(addr)

# Test IO Simple Functions
vibez.spill("Testing IO Simple...")
sus file_exists lit = io_file_exists_internal("test.csd")
assert_true(file_exists)
sus is_file lit = io_is_file_internal("test.csd")
assert_true(is_file)
sus is_dir lit = io_is_directory_internal("src")
assert_true(is_dir)

# Test Testz Compilation Support
vibez.spill("Testing Testz Functions...")
sus compile_result lit = attempt_compilation("test.csd")
assert_true(compile_result)
sus import_result lit = check_module_imports("testz")
assert_true(import_result)

print_test_summary()
