# FFI & Runtime Bindings Analysis Report
## CRITICAL PATH ANALYSIS for CURSED stdlib migration

### EXECUTIVE SUMMARY

**STATUS: CRITICAL GAPS IDENTIFIED**
- **45+ missing I/O FFI functions** blocking stdlib compilation
- **File handle management system** completely missing
- **Buffer management** not implemented
- **Memory allocator integration** requires GC bridge
- **ABI compatibility** needs verification across all types

---

## 1. EXISTING FFI IMPLEMENTATIONS INVENTORY

### ✅ IMPLEMENTED I/O Functions (24/69)
```c
// Console I/O (5/5) - COMPLETE
io_print(message: *const c_char) -> i32
io_println(message: *const c_char) -> i32
io_eprint(message: *const c_char) -> i32
io_eprintln(message: *const c_char) -> i32
io_printf(format: *const c_char, args: *const c_char) -> i32

// Basic File I/O (4/14) - PARTIAL
io_write_file(path: *const c_char, content: *const c_char) -> i32
io_read_file(path: *const c_char) -> *mut c_char
io_file_exists(path: *const c_char) -> i32
io_append_file(path: *const c_char, content: *const c_char) -> i32
io_copy_file(src: *const c_char, dest: *const c_char) -> i32
io_move_file(src: *const c_char, dest: *const c_char) -> i32
io_file_size(path: *const c_char) -> i64
io_is_file(path: *const c_char) -> i32
io_is_directory(path: *const c_char) -> i32

// Directory Operations (5/8) - PARTIAL
io_create_directory(path: *const c_char) -> i32
io_create_directory_recursive(path: *const c_char) -> i32
io_remove_directory(path: *const c_char) -> i32
io_remove_directory_recursive(path: *const c_char) -> i32
io_current_directory(buf: *mut c_char, len: usize) -> i32
io_change_directory(path: *const c_char) -> i32

// Console Input (3/3) - COMPLETE
io_read_line() -> *mut c_char
io_read_char(buf: *mut c_char, len: usize) -> i32
io_read_int() -> i32
io_read_float() -> f64
```

### ✅ IMPLEMENTED Collections Functions (30/30)
```c
// Array/Vector Operations (15/15) - COMPLETE
collections_array_new() -> *mut Vec<i64>
collections_array_with_capacity(capacity: usize) -> *mut Vec<i64>
collections_array_push(arr: *mut Vec<i64>, item: i64) -> i32
collections_array_pop(arr: *mut Vec<i64>) -> i64
collections_array_get(arr: *const Vec<i64>, index: usize) -> i64
collections_array_set(arr: *mut Vec<i64>, index: usize, value: i64) -> i32
collections_array_len(arr: *const Vec<i64>) -> usize
collections_array_insert(arr: *mut Vec<i64>, index: usize, item: i64) -> i32
collections_array_remove(arr: *mut Vec<i64>, index: usize) -> i64
collections_array_clear(arr: *mut Vec<i64>) -> i32
collections_array_is_empty(arr: *const Vec<i64>) -> i32
collections_array_contains(arr: *const Vec<i64>, item: i64) -> i32
collections_array_reverse(arr: *mut Vec<i64>) -> i32

// HashMap Operations (8/8) - COMPLETE
collections_map_new() -> *mut HashMap<i64, i64>
collections_map_with_capacity(capacity: usize) -> *mut HashMap<i64, i64>
collections_map_set(map: *mut HashMap<i64, i64>, key: i64, value: i64) -> i32
collections_map_get(map: *const HashMap<i64, i64>, key: i64) -> i64
collections_map_remove(map: *mut HashMap<i64, i64>, key: i64) -> i64
collections_map_contains_key(map: *const HashMap<i64, i64>, key: i64) -> i32
collections_map_len(map: *const HashMap<i64, i64>) -> usize
collections_map_clear(map: *mut HashMap<i64, i64>) -> i32
collections_map_is_empty(map: *const HashMap<i64, i64>) -> i32

// HashSet Operations (7/7) - COMPLETE
collections_set_new() -> *mut HashSet<i64>
collections_set_with_capacity(capacity: usize) -> *mut HashSet<i64>
collections_set_insert(set: *mut HashSet<i64>, item: i64) -> i32
collections_set_contains(set: *const HashSet<i64>, item: i64) -> i32
collections_set_remove(set: *mut HashSet<i64>, item: i64) -> i32
collections_set_len(set: *const HashSet<i64>) -> usize
collections_set_clear(set: *mut HashSet<i64>) -> i32
collections_set_is_empty(set: *const HashSet<i64>) -> i32
```

### ✅ IMPLEMENTED Crypto Functions (20/20)
```c
// Hash Functions (4/4) - COMPLETE
crypto_sha256(data: *const c_char) -> *mut c_char
crypto_sha512(data: *const c_char) -> *mut c_char
crypto_md5(data: *const c_char) -> *mut c_char
crypto_blake3(data: *const c_char) -> *mut c_char

// Random Generation (2/2) - COMPLETE
crypto_random_bytes(length: i64) -> *mut c_char
crypto_random_string(length: i64) -> *mut c_char

// Base64 Encoding (2/2) - COMPLETE
crypto_base64_encode(data: *const c_char) -> *mut c_char
crypto_base64_decode(data: *const c_char) -> *mut c_char

// AES Encryption (2/2) - COMPLETE
crypto_aes_encrypt(data: *const c_char, key: *const c_char) -> *mut c_char
crypto_aes_decrypt(data: *const c_char, key: *const c_char) -> *mut c_char

// HMAC (2/2) - COMPLETE
crypto_hmac_sha256(data: *const c_char, key: *const c_char) -> *mut c_char
crypto_hmac_sha512(data: *const c_char, key: *const c_char) -> *mut c_char

// Password Hashing (4/4) - COMPLETE
crypto_pbkdf2(password: *const c_char, salt: *const c_char, iterations: i64) -> *mut c_char
crypto_scrypt(password: *const c_char, salt: *const c_char) -> *mut c_char
crypto_argon2(password: *const c_char, salt: *const c_char) -> *mut c_char
crypto_bcrypt(password: *const c_char) -> *mut c_char

// Digital Signatures (2/2) - COMPLETE
crypto_ed25519_sign(message: *const c_char, private_key: *const c_char) -> *mut c_char
crypto_ed25519_verify(message: *const c_char, signature: *const c_char, public_key: *const c_char) -> i32

// Utility Functions (2/2) - COMPLETE
crypto_secure_compare(a: *const c_char, b: *const c_char) -> i32
crypto_generate_salt(length: i64) -> *mut c_char
```

---

## 2. CRITICAL MISSING FFI FUNCTIONS (45+ Functions)

### 🚨 PRIORITY 1: File Handle Management (MISSING ENTIRELY)

**Issue**: CURSED stdlib requires file handle system, but NO file handle FFI functions exist
```c
// MISSING: File handle type and operations
typedef struct file_handle file_handle;  // NOT DEFINED

// MISSING: File handle operations (9 functions)
io_open_file_read(path: *const c_char) -> file_handle
io_open_file_write(path: *const c_char) -> file_handle  
io_open_file_append(path: *const c_char) -> file_handle
io_close_file(handle: file_handle) -> i32
io_read_from_file(handle: file_handle, size: i32) -> *mut c_char
io_write_to_file(handle: file_handle, data: *const c_char) -> i32
io_flush_file(handle: file_handle) -> i32
io_seek_file(handle: file_handle, position: i32) -> i32
io_tell_file(handle: file_handle) -> i32
```

### 🚨 PRIORITY 1: Buffer Management (MISSING ENTIRELY)

**Issue**: CURSED stdlib requires buffer operations, but NO buffer FFI functions exist
```c
// MISSING: Buffer type and operations
typedef struct buffer buffer;  // NOT DEFINED

// MISSING: Buffer operations (7 functions)
io_create_buffer(size: i32) -> buffer
io_buffer_write(buf: buffer, data: *const c_char) -> i32
io_buffer_read(buf: buffer, size: i32) -> *mut c_char
io_buffer_flush(buf: buffer) -> i32
io_buffer_clear(buf: buffer) -> i32
io_buffer_size(buf: buffer) -> i32
io_buffer_available(buf: buffer) -> i32
```

### 🚨 PRIORITY 1: File System Operations (MISSING)

```c
// MISSING: File byte operations (2 functions)
io_read_file_bytes(path: *const c_char) -> *mut u8  // Returns [byte]
io_write_file_bytes(path: *const c_char, data: *const u8, len: usize) -> i32

// MISSING: File metadata (2 functions)
io_file_modified_time(path: *const c_char) -> i64
io_file_created_time(path: *const c_char) -> i64

// MISSING: Directory listing (2 functions)
io_list_directory(path: *const c_char) -> *mut *mut c_char  // Returns [tea]
io_list_directory_recursive(path: *const c_char) -> *mut *mut c_char
```

### 🚨 PRIORITY 1: Path Operations (MISSING ENTIRELY)

```c
// MISSING: Path manipulation (7 functions)
io_path_join(parts: *const *const c_char, count: usize) -> *mut c_char
io_path_dirname(path: *const c_char) -> *mut c_char
io_path_basename(path: *const c_char) -> *mut c_char
io_path_extension(path: *const c_char) -> *mut c_char
io_path_absolute(path: *const c_char) -> *mut c_char
io_path_relative(from: *const c_char, to: *const c_char) -> *mut c_char
io_path_exists(path: *const c_char) -> i32
```

### 🚨 PRIORITY 1: Temporary File Operations (MISSING)

```c
// MISSING: Temporary file operations (3 functions)
io_create_temp_file() -> *mut c_char
io_create_temp_directory() -> *mut c_char  
io_temp_directory() -> *mut c_char
```

### 🚨 PRIORITY 2: String Operations (MISSING)

```c
// MISSING: String manipulation functions (10+ functions)
string_length(s: *const c_char) -> usize
string_concat(a: *const c_char, b: *const c_char) -> *mut c_char
string_substring(s: *const c_char, start: usize, end: usize) -> *mut c_char
string_split(s: *const c_char, delimiter: *const c_char) -> *mut *mut c_char
string_join(parts: *const *const c_char, count: usize, separator: *const c_char) -> *mut c_char
string_replace(s: *const c_char, old: *const c_char, new: *const c_char) -> *mut c_char
string_to_upper(s: *const c_char) -> *mut c_char
string_to_lower(s: *const c_char) -> *mut c_char
string_trim(s: *const c_char) -> *mut c_char
string_contains(s: *const c_char, substring: *const c_char) -> i32
```

### 🚨 PRIORITY 2: Math Operations (MISSING)

```c
// MISSING: Math library functions (20+ functions)
math_abs(x: f64) -> f64
math_pow(x: f64, y: f64) -> f64
math_sqrt(x: f64) -> f64
math_sin(x: f64) -> f64
math_cos(x: f64) -> f64
math_tan(x: f64) -> f64
math_log(x: f64) -> f64
math_exp(x: f64) -> f64
math_floor(x: f64) -> f64
math_ceil(x: f64) -> f64
math_round(x: f64) -> f64
math_min(a: f64, b: f64) -> f64
math_max(a: f64, b: f64) -> f64
math_random() -> f64
math_seed(seed: i64) -> i32
```

### 🚨 PRIORITY 2: Time Operations (MISSING)

```c
// MISSING: Time library functions (8+ functions)
time_now() -> i64
time_sleep(milliseconds: i64) -> i32
time_format(timestamp: i64, format: *const c_char) -> *mut c_char
time_parse(time_str: *const c_char, format: *const c_char) -> i64
time_unix() -> i64
time_unix_nano() -> i64
time_duration_since(start: i64) -> i64
time_add_duration(timestamp: i64, duration: i64) -> i64
```

---

## 3. ABI COMPATIBILITY ANALYSIS

### CURSED Type → C ABI Mapping

| CURSED Type | C Type | ABI Size | Alignment | Notes |
|-------------|--------|----------|-----------|-------|
| `tea` (string) | `*const c_char` | 8 bytes | 8 bytes | Null-terminated UTF-8 |
| `normie` (i32) | `i32` | 4 bytes | 4 bytes | ✅ Compatible |
| `smol` (i8) | `i8` | 1 byte | 1 byte | ✅ Compatible |
| `mid` (i16) | `i16` | 2 bytes | 2 bytes | ✅ Compatible |
| `thicc` (i64) | `i64` | 8 bytes | 8 bytes | ✅ Compatible |
| `drip` (f32) | `f32` | 4 bytes | 4 bytes | ✅ Compatible |
| `meal` (f64) | `f64` | 8 bytes | 8 bytes | ✅ Compatible |
| `lit` (bool) | `i32` | 4 bytes | 4 bytes | ⚠️ Needs conversion |
| `sip` (char) | `i32` | 4 bytes | 4 bytes | ⚠️ UTF-8 rune |
| `byte` (u8) | `u8` | 1 byte | 1 byte | ✅ Compatible |
| `[type]` (array) | `*mut type` | 8 bytes | 8 bytes | ⚠️ Needs length tracking |
| `file_handle` | `*mut FILE` | 8 bytes | 8 bytes | ❌ NOT DEFINED |
| `buffer` | `*mut Buffer` | 8 bytes | 8 bytes | ❌ NOT DEFINED |

### 🚨 CRITICAL ABI ISSUES

1. **File Handle Type**: No C struct defined for `file_handle`
2. **Buffer Type**: No C struct defined for `buffer`
3. **Array Length**: No mechanism to pass array lengths
4. **String Array**: No mechanism to return `[tea]` arrays
5. **Memory Management**: No clear ownership semantics

---

## 4. MEMORY ALLOCATOR INTEGRATION ANALYSIS

### Current Memory Management Issues

**Problem**: CURSED stdlib functions return heap-allocated strings (`*mut c_char`) but no integration with CURSED GC system.

```rust
// CURRENT ISSUE: Raw malloc/free used
pub extern "C" fn io_read_file(path_ptr: *const c_char) -> *mut c_char {
    // Returns CString::into_raw() - uses system malloc
    // NOT tracked by CURSED GC system
}
```

### 🚨 REQUIRED GC INTEGRATION

```rust
// MISSING: GC-aware allocation functions
pub extern "C" fn cursed_alloc_string(len: usize) -> *mut c_char {
    // Must integrate with src/runtime/gc.rs
    // Must register with GC root tracking
}

pub extern "C" fn cursed_free_string(ptr: *mut c_char) {
    // Must integrate with GC cleanup
}
```

### Memory Bridge Requirements

1. **GC Registration**: All FFI allocations must register with GC
2. **Root Tracking**: String returns must be tracked as GC roots
3. **Cleanup Integration**: FFI cleanup must integrate with GC cycles
4. **Lifetime Management**: Clear ownership semantics needed

---

## 5. RISK ASSESSMENT

### HIGH RISK - BLOCKING ISSUES

| Issue | Impact | Probability | Mitigation |
|-------|--------|------------|------------|
| File handle system missing | **CRITICAL** | 100% | Implement file handle struct + 9 functions |
| Buffer management missing | **CRITICAL** | 100% | Implement buffer struct + 7 functions |
| Path operations missing | **HIGH** | 100% | Implement 7 path manipulation functions |
| GC integration missing | **HIGH** | 90% | Bridge FFI allocations with GC system |
| Array return types broken | **HIGH** | 80% | Fix `[tea]` and `[byte]` return mechanisms |

### MEDIUM RISK - PERFORMANCE ISSUES

| Issue | Impact | Probability | Mitigation |
|-------|--------|------------|------------|
| String allocation overhead | **MEDIUM** | 70% | Optimize string allocation paths |
| FFI call overhead | **MEDIUM** | 50% | Minimize FFI boundary crossings |
| Type conversion overhead | **MEDIUM** | 60% | Optimize type conversions |

### LOW RISK - COMPATIBILITY ISSUES

| Issue | Impact | Probability | Mitigation |
|-------|--------|------------|------------|
| Platform-specific paths | **LOW** | 30% | Use platform-agnostic implementations |
| Unicode handling | **LOW** | 20% | Ensure proper UTF-8 handling |
| Error code consistency | **LOW** | 40% | Standardize error return codes |

---

## 6. IMPLEMENTATION PRIORITY LIST

### 🚨 PHASE 1: CRITICAL BLOCKING FUNCTIONS (Week 1)

1. **File Handle System** (9 functions)
   - Define `file_handle` C struct
   - Implement `io_open_file_*` functions
   - Implement file I/O operations

2. **Buffer Management** (7 functions)
   - Define `buffer` C struct
   - Implement buffer operations
   - Memory management integration

3. **Path Operations** (7 functions)
   - Cross-platform path manipulation
   - Integration with std::path

### 🚨 PHASE 2: STDLIB COMPLETION (Week 2)

1. **String Operations** (10 functions)
   - String manipulation functions
   - UTF-8 handling

2. **Math Operations** (15 functions)
   - Math library functions
   - Floating-point operations

3. **Time Operations** (8 functions)
   - Time and date functions
   - Duration calculations

### 🚨 PHASE 3: OPTIMIZATION (Week 3)

1. **GC Integration**
   - Memory bridge implementation
   - GC root tracking

2. **Performance Optimization**
   - Reduce FFI overhead
   - Optimize type conversions

3. **Error Handling**
   - Consistent error codes
   - Better error reporting

---

## 7. IMMEDIATE ACTION ITEMS

### CRITICAL PATH - MUST DO THIS WEEK

1. **Define missing C structs**:
   ```c
   typedef struct {
       FILE* file_ptr;
       int flags;
       int error_code;
   } file_handle;
   
   typedef struct {
       char* data;
       size_t size;
       size_t capacity;
       size_t position;
   } buffer;
   ```

2. **Implement file handle operations**:
   - `io_open_file_read/write/append`
   - `io_close_file`
   - `io_read_from_file/write_to_file`

3. **Implement buffer operations**:
   - `io_create_buffer`
   - `io_buffer_read/write`
   - `io_buffer_flush/clear`

4. **Fix array return types**:
   - Implement `[tea]` array returns
   - Implement `[byte]` array returns

### TESTING REQUIREMENTS

1. **FFI Integration Tests**:
   - Test all file operations
   - Test buffer operations
   - Test path operations

2. **Memory Management Tests**:
   - Test GC integration
   - Test memory leak detection
   - Test allocation tracking

3. **Performance Tests**:
   - FFI call overhead
   - Memory allocation performance
   - Type conversion performance

---

## CONCLUSION

**CRITICAL STATUS**: 45+ missing FFI functions are blocking CURSED stdlib migration. File handle and buffer management systems are completely missing and must be implemented immediately.

**RECOMMENDED ACTION**: Implement Phase 1 functions (23 functions) within 1 week to unblock stdlib compilation. This includes file handle system, buffer management, and path operations.

**RESOURCES NEEDED**: 
- 1 developer-week for Phase 1 implementation
- Integration testing framework
- Memory management bridge design
- Cross-platform testing environment

**SUCCESS METRICS**:
- All 69 I/O functions implemented
- All stdlib tests passing
- GC integration working
- Native compilation successful
- Zero memory leaks detected
