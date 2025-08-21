# 🔗 CURSED Interoperability Guide

CURSED provides seamless interoperability with C libraries and other languages through its Foreign Function Interface (FFI) system.

## 📚 Table of Contents

- [C Interoperability](#c-interoperability)
- [Memory Management](#memory-management)
- [Type Mappings](#type-mappings)
- [Calling Conventions](#calling-conventions)
- [Common Patterns](#common-patterns)
- [Best Practices](#best-practices)
- [Examples](#examples)

## 🎯 C Interoperability

### Basic External Function Declaration

```cursed
# Declare external C functions
extern "C" {
    slay malloc(size drip) *lit
    slay free(ptr *lit)
    slay strlen(s *lit) drip
    slay strcpy(dest *lit, src *lit) *lit
    slay printf(format *lit, ...) drip
}

# Use C functions directly
sus memory *lit = malloc(1024)
printf("Allocated %d bytes at %p\n", 1024, memory)
free(memory)
```

### Working with C Libraries

```cursed
# Link with external libraries
#[link(name = "sqlite3")]
extern "C" {
    squad sqlite3 {}
    
    slay sqlite3_open(filename *lit, db **sqlite3) drip
    slay sqlite3_close(db *sqlite3) drip
    slay sqlite3_exec(db *sqlite3, sql *lit, callback *lit, arg *lit, errmsg **lit) drip
    slay sqlite3_errmsg(db *sqlite3) *lit
}

# Example usage
sus db *sqlite3 = null
sus result drip = sqlite3_open("database.db", &db)
ready (result == 0) {
    vibez.spill("Database opened successfully")
    sqlite3_close(db)
} otherwise {
    vibez.spill("Failed to open database")
}
```

## 🧠 Memory Management

### Manual Memory Management

```cursed
# Allocate and manage C memory
slay allocate_buffer(size drip) *lit {
    sus buffer *lit = malloc(size)
    ready (buffer == null) {
        yikes "memory allocation failed"
    }
    damn buffer
}

slay safe_free(ptr *lit) {
    ready (ptr != null) {
        free(ptr)
    }
}

# RAII pattern for automatic cleanup
squad CBuffer {
    data *lit
    size drip
}

impl Drop for CBuffer {
    slay drop() {
        ready (self.data != null) {
            free(self.data)
            self.data = null
        }
    }
}
```

### Arena Allocators for C Interop

```cursed
yeet "memoryz"

# Use arena allocators for C interop
slay with_c_arena<T>(size drip, func slay(*memoryz.Arena) T) T {
    sus arena memoryz.Arena = memoryz.new_arena(size)
    defer memoryz.destroy_arena(&arena)
    
    damn func(&arena)
}

# Example: Working with C strings
slay process_c_strings() {
    with_c_arena(4096, slay(arena *memoryz.Arena) {
        sus c_string *lit = memoryz.arena_alloc_cstring(arena, "Hello from CURSED!")
        printf("C String: %s\n", c_string)
        # No manual cleanup needed - arena handles it
    })
}
```

## 🔄 Type Mappings

### Primitive Type Conversions

```cursed
# CURSED to C type mappings
squad TypeMappings {
    # Integer types
    cursed_drip_to_c: slay(value drip) i32 { damn value as i32 }
    c_int_to_cursed: slay(value i32) drip { damn value as drip }
    
    # Floating point
    cursed_meal_to_c: slay(value meal) f64 { damn value as f64 }
    c_double_to_cursed: slay(value f64) meal { damn value as meal }
    
    # Boolean
    cursed_lit_to_c: slay(value lit) i32 { damn ready (value) { 1 } otherwise { 0 } }
    c_bool_to_cursed: slay(value i32) lit { damn value != 0 }
    
    # Pointers
    cursed_ptr_to_c: slay<T>(ptr *T) *lit { damn ptr as *lit }
    c_ptr_to_cursed: slay<T>(ptr *lit) *T { damn ptr as *T }
}
```

### String Interoperability

```cursed
# String conversion utilities
squad StringInterop {
    # Convert CURSED string to null-terminated C string
    slay to_c_string(s tea) *lit {
        sus c_str *lit = malloc(stringz.len(s) + 1)
        ready (c_str == null) {
            damn null
        }
        
        sus bytes []lit = stringz.to_bytes(s)
        memoryz.copy(c_str, bytes.ptr, len(bytes))
        memoryz.set_byte(c_str + len(bytes), 0)  # null terminator
        
        damn c_str
    }
    
    # Convert C string to CURSED string
    slay from_c_string(c_str *lit) tea {
        ready (c_str == null) {
            damn ""
        }
        
        sus length drip = strlen(c_str)
        sus bytes []lit = []
        sus i drip = 0
        
        bestie (i < length) {
            bytes = arrayz.append(bytes, memoryz.get_byte(c_str + i))
            i = i + 1
        }
        
        damn stringz.from_bytes(bytes)
    }
}
```

### Struct Interoperability

```cursed
# C-compatible struct layouts
#[repr(C)]
squad CCompatibleStruct {
    id i32
    name [64]lit  # Fixed-size array for C compatibility
    value f64
    flags u32
}

# Conversion functions
slay to_c_struct(cursed_data MyData) CCompatibleStruct {
    sus c_struct CCompatibleStruct = {}
    c_struct.id = cursed_data.id as i32
    c_struct.value = cursed_data.value as f64
    c_struct.flags = cursed_data.flags as u32
    
    # Copy string with bounds checking
    sus name_bytes []lit = stringz.to_bytes(cursed_data.name)
    sus copy_len drip = mathz.min(len(name_bytes), 63)  # Leave space for null terminator
    memoryz.copy(&c_struct.name[0], name_bytes.ptr, copy_len)
    c_struct.name[copy_len] = 0
    
    damn c_struct
}
```

## 📞 Calling Conventions

### Function Pointers and Callbacks

```cursed
# Define callback types
type CallbackFunc = slay(data *lit, size drip) drip

# C function that takes callbacks
extern "C" {
    slay process_data(data *lit, size drip, callback CallbackFunc) drip
}

# CURSED callback implementation
slay my_callback(data *lit, size drip) drip {
    vibez.spill("Processing", size, "bytes of data")
    # Process the data...
    damn 0  # Success
}

# Use the callback
sus result drip = process_data(some_data, data_size, my_callback)
```

### Variadic Functions

```cursed
# Working with C variadic functions
extern "C" {
    slay printf(format *lit, ...) drip
    slay sprintf(buffer *lit, format *lit, ...) drip
}

# Type-safe wrappers for common cases
slay print_int(format tea, value drip) {
    sus c_format *lit = StringInterop.to_c_string(format)
    defer free(c_format)
    
    printf(c_format, value as i32)
}

slay print_string(format tea, value tea) {
    sus c_format *lit = StringInterop.to_c_string(format)
    sus c_value *lit = StringInterop.to_c_string(value)
    defer {
        free(c_format)
        free(c_value)
    }
    
    printf(c_format, c_value)
}
```

## 🏗️ Common Patterns

### RAII Wrapper Pattern

```cursed
# Automatic resource management for C resources
squad FileHandle {
    fp *lit  # FILE* from C
}

impl FileHandle {
    slay open(filename tea, mode tea) yikes<FileHandle> {
        sus c_filename *lit = StringInterop.to_c_string(filename)
        sus c_mode *lit = StringInterop.to_c_string(mode)
        defer {
            free(c_filename)
            free(c_mode)
        }
        
        sus fp *lit = fopen(c_filename, c_mode)
        ready (fp == null) {
            yikes "failed to open file"
        }
        
        damn FileHandle{fp: fp}
    }
    
    slay read(buffer []lit) yikes<drip> {
        ready (self.fp == null) {
            yikes "file handle is closed"
        }
        
        sus bytes_read drip = fread(buffer.ptr, 1, len(buffer), self.fp)
        damn bytes_read
    }
    
    slay write(data []lit) yikes<drip> {
        ready (self.fp == null) {
            yikes "file handle is closed"
        }
        
        sus bytes_written drip = fwrite(data.ptr, 1, len(data), self.fp)
        damn bytes_written
    }
}

impl Drop for FileHandle {
    slay drop() {
        ready (self.fp != null) {
            fclose(self.fp)
            self.fp = null
        }
    }
}
```

### Error Code Translation

```cursed
# Translate C error codes to CURSED errors
enum CError {
    Success = 0,
    InvalidArgument = 1,
    OutOfMemory = 2,
    IOError = 3,
    PermissionDenied = 4
}

slay translate_c_error(code drip) yikes<lit> {
    sick (code) {
        when 0 -> damn based  # Success
        when 1 -> yikes "invalid argument"
        when 2 -> yikes "out of memory"
        when 3 -> yikes "I/O error"
        when 4 -> yikes "permission denied"
        when _ -> yikes stringz.format("unknown error code: %d", code)
    }
}

# Use with C functions
slay safe_c_function_call() yikes<lit> {
    sus result drip = some_c_function()
    damn translate_c_error(result)
}
```

## ✅ Best Practices

### 1. Memory Safety

```cursed
# Always check for null pointers
slay safe_c_call(ptr *lit) yikes<drip> {
    ready (ptr == null) {
        yikes "null pointer passed to C function"
    }
    
    sus result drip = c_function_that_might_fail(ptr)
    ready (result < 0) {
        yikes "C function returned error"
    }
    
    damn result
}

# Use RAII for automatic cleanup
slay with_c_resource<T>(create_func slay() *lit, cleanup_func slay(*lit), 
                       use_func slay(*lit) T) yikes<T> {
    sus resource *lit = create_func()
    ready (resource == null) {
        yikes "failed to create resource"
    }
    defer cleanup_func(resource)
    
    damn use_func(resource)
}
```

### 2. String Handling

```cursed
# Safe string conversion with bounds checking
slay safe_string_to_c(s tea, max_length drip) *lit {
    ready (stringz.len(s) > max_length) {
        # Truncate or return error
        yikes "string too long for C buffer"
    }
    
    damn StringInterop.to_c_string(s)
}

# Always validate C strings before conversion
slay validate_c_string(c_str *lit, max_length drip) yikes<tea> {
    ready (c_str == null) {
        yikes "null C string"
    }
    
    sus length drip = strnlen(c_str, max_length + 1)
    ready (length > max_length) {
        yikes "C string too long or not null-terminated"
    }
    
    damn StringInterop.from_c_string(c_str)
}
```

### 3. Error Handling

```cursed
# Comprehensive error handling for C interop
squad CInteropResult<T> {
    success lit
    value T
    c_error_code drip
    error_message tea
}

slay safe_c_interop<T>(operation slay() (T, drip)) CInteropResult<T> {
    sus (value, error_code) = operation()
    
    ready (error_code == 0) {
        damn CInteropResult{
            success: based,
            value: value,
            c_error_code: 0,
            error_message: ""
        }
    } otherwise {
        damn CInteropResult{
            success: false,
            value: T{},  # Zero value
            c_error_code: error_code,
            error_message: get_c_error_message(error_code)
        }
    }
}
```

## 🌐 Examples

### SQLite Integration

```cursed
yeet "stringz"

# Complete SQLite wrapper
squad Database {
    db *sqlite3
    is_open lit
}

impl Database {
    slay open(path tea) yikes<Database> {
        sus c_path *lit = StringInterop.to_c_string(path)
        defer free(c_path)
        
        sus db *sqlite3 = null
        sus result drip = sqlite3_open(c_path, &db)
        
        ready (result != 0) {
            sus error_msg tea = StringInterop.from_c_string(sqlite3_errmsg(db))
            sqlite3_close(db)  # Clean up even on error
            yikes error_msg
        }
        
        damn Database{db: db, is_open: based}
    }
    
    slay execute(sql tea) yikes<lit> {
        ready (!self.is_open) {
            yikes "database is not open"
        }
        
        sus c_sql *lit = StringInterop.to_c_string(sql)
        defer free(c_sql)
        
        sus result drip = sqlite3_exec(self.db, c_sql, null, null, null)
        ready (result != 0) {
            sus error_msg tea = StringInterop.from_c_string(sqlite3_errmsg(self.db))
            yikes error_msg
        }
        
        damn based
    }
}

impl Drop for Database {
    slay drop() {
        ready (self.is_open && self.db != null) {
            sqlite3_close(self.db)
            self.is_open = false
            self.db = null
        }
    }
}

# Usage example
slay database_example() yikes<lit> {
    sus db Database = Database.open("example.db") fam {
        when _ -> yikes "failed to open database"
    }
    
    db.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)") fam {
        when _ -> yikes "failed to create table"
    }
    
    db.execute("INSERT INTO users (name) VALUES ('Alice')") fam {
        when _ -> yikes "failed to insert user"
    }
    
    vibez.spill("Database operations completed successfully")
    damn based
}
```

### OpenSSL Cryptography

```cursed
# OpenSSL integration example
#[link(name = "ssl")]
#[link(name = "crypto")]
extern "C" {
    slay SSL_library_init() drip
    slay EVP_sha256() *lit
    slay EVP_DigestInit_ex(ctx *lit, type *lit, impl *lit) drip
    slay EVP_DigestUpdate(ctx *lit, data *lit, count drip) drip
    slay EVP_DigestFinal_ex(ctx *lit, md *lit, s *drip) drip
    slay EVP_MD_CTX_new() *lit
    slay EVP_MD_CTX_free(ctx *lit)
}

squad HashContext {
    ctx *lit
}

impl HashContext {
    slay new() yikes<HashContext> {
        SSL_library_init()
        
        sus ctx *lit = EVP_MD_CTX_new()
        ready (ctx == null) {
            yikes "failed to create hash context"
        }
        
        sus result drip = EVP_DigestInit_ex(ctx, EVP_sha256(), null)
        ready (result != 1) {
            EVP_MD_CTX_free(ctx)
            yikes "failed to initialize hash context"
        }
        
        damn HashContext{ctx: ctx}
    }
    
    slay update(data []lit) yikes<lit> {
        sus result drip = EVP_DigestUpdate(self.ctx, data.ptr, len(data))
        ready (result != 1) {
            yikes "failed to update hash"
        }
        damn based
    }
    
    slay finalize() yikes<[]lit> {
        sus hash [32]lit = {}  # SHA-256 produces 32 bytes
        sus hash_len drip = 0
        
        sus result drip = EVP_DigestFinal_ex(self.ctx, &hash[0], &hash_len)
        ready (result != 1) {
            yikes "failed to finalize hash"
        }
        
        # Convert to slice
        sus hash_slice []lit = []
        sus i drip = 0
        bestie (i < hash_len) {
            hash_slice = arrayz.append(hash_slice, hash[i])
            i = i + 1
        }
        
        damn hash_slice
    }
}

impl Drop for HashContext {
    slay drop() {
        ready (self.ctx != null) {
            EVP_MD_CTX_free(self.ctx)
            self.ctx = null
        }
    }
}

# Usage example
slay hash_example() yikes<lit> {
    sus ctx HashContext = HashContext.new() fam {
        when _ -> yikes "failed to create hash context"
    }
    
    sus data tea = "Hello, CURSED cryptography!"
    sus data_bytes []lit = stringz.to_bytes(data)
    
    ctx.update(data_bytes) fam {
        when _ -> yikes "failed to update hash"
    }
    
    sus hash []lit = ctx.finalize() fam {
        when _ -> yikes "failed to finalize hash"
    }
    
    vibez.spill("SHA-256 hash computed successfully:", len(hash), "bytes")
    damn based
}
```

## 🚀 Performance Tips

1. **Minimize Conversions**: Cache converted strings and avoid repeated conversions
2. **Use Arena Allocators**: For temporary C interop data
3. **Batch Operations**: Group multiple C calls together
4. **Profile Memory Usage**: Monitor C heap usage with Valgrind
5. **Zero-Copy When Possible**: Use direct pointer access for large data

## 🔍 Debugging

```cursed
# Debug helpers for C interop
#[cfg(debug)]
squad CDebugHelpers {
    slay trace_malloc(size drip) *lit {
        sus ptr *lit = malloc(size)
        vibez.spill("DEBUG: malloc(", size, ") -> ", ptr)
        damn ptr
    }
    
    slay trace_free(ptr *lit) {
        vibez.spill("DEBUG: free(", ptr, ")")
        free(ptr)
    }
    
    slay validate_pointer(ptr *lit, name tea) yikes<lit> {
        ready (ptr == null) {
            yikes stringz.format("null pointer: %s", name)
        }
        damn based
    }
}
```

---

**The CURSED FFI system provides safe, efficient interoperability with C while maintaining memory safety and error handling! 🔗**
