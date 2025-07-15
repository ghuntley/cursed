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
#include <stddef.h>

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
