#!/bin/bash

# Create minimal C shims for self-hosting
# This script identifies essential FFI functions and creates minimal C implementations

echo "Creating minimal C shims for self-hosting..."

# Create runtime/minimal_shims.h
cat > runtime/minimal_shims.h << 'EOF'
/*
 * Minimal C Shims for CURSED Self-Hosting
 * 
 * This header provides minimal C shims that replace full FFI dependencies
 * while maintaining compatibility with the CURSED runtime system.
 */

#ifndef CURSED_MINIMAL_SHIMS_H
#define CURSED_MINIMAL_SHIMS_H

#include <stdint.h>
#include <stdbool.h>

// Essential I/O operations
int cursed_print(const char* str);
int cursed_println(const char* str);
char* cursed_read_line(void);

// Basic memory management
void* cursed_malloc(size_t size);
void cursed_free(void* ptr);

// String operations
int cursed_string_length(const char* str);
char* cursed_string_concat(const char* a, const char* b);
int cursed_string_compare(const char* a, const char* b);

// File operations
int cursed_file_exists(const char* path);
char* cursed_file_read(const char* path);
int cursed_file_write(const char* path, const char* content);

// Network operations (minimal)
int cursed_net_tcp_create(void);
int cursed_net_tcp_connect(int handle, const char* address, int port);
int cursed_net_tcp_send(int handle, const char* data);
char* cursed_net_tcp_recv(int handle, int max_size);
void cursed_net_tcp_close(int handle);

// Process operations
int cursed_process_spawn(const char* command, char* const argv[]);
int cursed_process_wait(int pid);
int cursed_process_kill(int pid);

// Time operations
uint64_t cursed_time_now_ms(void);
void cursed_time_sleep_ms(uint64_t ms);

// Crypto operations (minimal)
char* cursed_crypto_sha256(const char* data);
char* cursed_crypto_random_bytes(int length);

#endif // CURSED_MINIMAL_SHIMS_H
EOF

# Create runtime/minimal_shims.c
cat > runtime/minimal_shims.c << 'EOF'
/*
 * Minimal C Shims for CURSED Self-Hosting
 * 
 * This file provides minimal C implementations that replace full FFI dependencies
 * while maintaining essential functionality for self-hosting.
 */

#include "minimal_shims.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/wait.h>
#include <time.h>
#include <openssl/sha.h>
#include <openssl/rand.h>

// Essential I/O operations
int cursed_print(const char* str) {
    if (!str) return -1;
    return printf("%s", str);
}

int cursed_println(const char* str) {
    if (!str) return -1;
    return printf("%s\n", str);
}

char* cursed_read_line(void) {
    static char buffer[4096];
    if (fgets(buffer, sizeof(buffer), stdin)) {
        // Remove trailing newline
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len-1] == '\n') {
            buffer[len-1] = '\0';
        }
        return strdup(buffer);
    }
    return NULL;
}

// Basic memory management
void* cursed_malloc(size_t size) {
    return malloc(size);
}

void cursed_free(void* ptr) {
    free(ptr);
}

// String operations
int cursed_string_length(const char* str) {
    return str ? strlen(str) : 0;
}

char* cursed_string_concat(const char* a, const char* b) {
    if (!a || !b) return NULL;
    
    size_t len_a = strlen(a);
    size_t len_b = strlen(b);
    char* result = malloc(len_a + len_b + 1);
    
    if (result) {
        strcpy(result, a);
        strcat(result, b);
    }
    
    return result;
}

int cursed_string_compare(const char* a, const char* b) {
    if (!a || !b) return -1;
    return strcmp(a, b);
}

// File operations
int cursed_file_exists(const char* path) {
    if (!path) return 0;
    struct stat st;
    return stat(path, &st) == 0;
}

char* cursed_file_read(const char* path) {
    if (!path) return NULL;
    
    FILE* file = fopen(path, "r");
    if (!file) return NULL;
    
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    char* content = malloc(size + 1);
    if (content) {
        fread(content, 1, size, file);
        content[size] = '\0';
    }
    
    fclose(file);
    return content;
}

int cursed_file_write(const char* path, const char* content) {
    if (!path || !content) return -1;
    
    FILE* file = fopen(path, "w");
    if (!file) return -1;
    
    int result = fprintf(file, "%s", content);
    fclose(file);
    
    return result >= 0 ? 0 : -1;
}

// Network operations (minimal)
int cursed_net_tcp_create(void) {
    return socket(AF_INET, SOCK_STREAM, 0);
}

int cursed_net_tcp_connect(int handle, const char* address, int port) {
    if (handle < 0 || !address) return -1;
    
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    addr.sin_addr.s_addr = inet_addr(address);
    
    return connect(handle, (struct sockaddr*)&addr, sizeof(addr));
}

int cursed_net_tcp_send(int handle, const char* data) {
    if (handle < 0 || !data) return -1;
    return send(handle, data, strlen(data), 0);
}

char* cursed_net_tcp_recv(int handle, int max_size) {
    if (handle < 0 || max_size <= 0) return NULL;
    
    char* buffer = malloc(max_size + 1);
    if (!buffer) return NULL;
    
    int received = recv(handle, buffer, max_size, 0);
    if (received > 0) {
        buffer[received] = '\0';
        return buffer;
    }
    
    free(buffer);
    return NULL;
}

void cursed_net_tcp_close(int handle) {
    if (handle >= 0) {
        close(handle);
    }
}

// Process operations
int cursed_process_spawn(const char* command, char* const argv[]) {
    if (!command) return -1;
    
    pid_t pid = fork();
    if (pid == 0) {
        // Child process
        execvp(command, argv);
        exit(127);
    }
    
    return pid;
}

int cursed_process_wait(int pid) {
    if (pid < 0) return -1;
    
    int status;
    waitpid(pid, &status, 0);
    return WEXITSTATUS(status);
}

int cursed_process_kill(int pid) {
    if (pid < 0) return -1;
    return kill(pid, SIGTERM);
}

// Time operations
uint64_t cursed_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

void cursed_time_sleep_ms(uint64_t ms) {
    usleep(ms * 1000);
}

// Crypto operations (minimal)
char* cursed_crypto_sha256(const char* data) {
    if (!data) return NULL;
    
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, data, strlen(data));
    SHA256_Final(hash, &sha256);
    
    char* result = malloc(SHA256_DIGEST_LENGTH * 2 + 1);
    if (result) {
        for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
            sprintf(result + i * 2, "%02x", hash[i]);
        }
        result[SHA256_DIGEST_LENGTH * 2] = '\0';
    }
    
    return result;
}

char* cursed_crypto_random_bytes(int length) {
    if (length <= 0) return NULL;
    
    char* buffer = malloc(length);
    if (buffer && RAND_bytes((unsigned char*)buffer, length) == 1) {
        return buffer;
    }
    
    free(buffer);
    return NULL;
}
EOF

echo "Created minimal C shims in runtime/minimal_shims.{h,c}"

# Create pure CURSED implementations for high-level operations
mkdir -p stdlib/pure_cursed_runtime

cat > stdlib/pure_cursed_runtime/mod.csd << 'EOF'
# Pure CURSED Runtime Bridge
# Provides essential runtime functions using minimal C shims

slay print(message tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_print(message) == 0
}

slay println(message tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_println(message) == 0
}

slay read_line() tea {
    # Call minimal C shim instead of full FFI
    damn cursed_read_line()
}

slay string_length(s tea) normie {
    # Call minimal C shim instead of full FFI
    damn cursed_string_length(s)
}

slay string_concat(a tea, b tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_string_concat(a, b)
}

slay file_exists(path tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_file_exists(path) == 1
}

slay file_read(path tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_file_read(path)
}

slay file_write(path tea, content tea) lit {
    # Call minimal C shim instead of full FFI
    damn cursed_file_write(path, content) == 0
}

slay time_now_ms() normie {
    # Call minimal C shim instead of full FFI
    damn cursed_time_now_ms()
}

slay sleep_ms(ms normie) {
    # Call minimal C shim instead of full FFI
    cursed_time_sleep_ms(ms)
}

slay sha256(data tea) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_crypto_sha256(data)
}

slay random_bytes(length normie) tea {
    # Call minimal C shim instead of full FFI
    damn cursed_crypto_random_bytes(length)
}
EOF

cat > stdlib/pure_cursed_runtime/test_pure_cursed_runtime.csd << 'EOF'
yeet "testz"
yeet "pure_cursed_runtime"

test_start("pure CURSED runtime bridge tests")

# Test basic I/O
assert_true(print("test message"))
assert_true(println("test message with newline"))

# Test string operations
assert_eq_int(string_length("hello"), 5)
assert_eq_string(string_concat("hello", " world"), "hello world")

# Test file operations
assert_true(file_write("/tmp/test.txt", "test content"))
assert_true(file_exists("/tmp/test.txt"))
assert_eq_string(file_read("/tmp/test.txt"), "test content")

# Test time operations
sus start_time := time_now_ms()
sleep_ms(10)
sus end_time := time_now_ms()
assert_true(end_time > start_time)

# Test crypto operations
sus hash := sha256("test data")
assert_true(string_length(hash) == 64)  # SHA256 produces 64 hex characters

sus random_data := random_bytes(16)
assert_true(string_length(random_data) == 16)

print_test_summary()
EOF

echo "Created pure CURSED runtime bridge in stdlib/pure_cursed_runtime/"

# Update build.rs to use minimal shims
cat >> build.rs << 'EOF'

// Link minimal C shims for self-hosting
println!("cargo:rustc-link-lib=static=cursed_minimal_shims");
println!("cargo:rustc-link-search=native=runtime");
println!("cargo:rustc-link-lib=crypto");  // For OpenSSL crypto functions
EOF

echo "Updated build.rs to link minimal shims"

# Create compilation script for minimal shims
cat > scripts/compile_minimal_shims.sh << 'EOF'
#!/bin/bash

echo "Compiling minimal C shims..."

# Compile minimal shims to static library
cd runtime
gcc -c -fPIC minimal_shims.c -o minimal_shims.o
ar rcs libcursed_minimal_shims.a minimal_shims.o

echo "Compiled minimal shims to runtime/libcursed_minimal_shims.a"
EOF

chmod +x scripts/compile_minimal_shims.sh

echo "Created compilation script: scripts/compile_minimal_shims.sh"
echo ""
echo "To use the minimal shims:"
echo "1. ./scripts/compile_minimal_shims.sh"
echo "2. cargo build"
echo "3. Test with: cargo run --bin cursed stdlib/pure_cursed_runtime/test_pure_cursed_runtime.csd"
