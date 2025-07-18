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
// Simple SHA256 implementation without OpenSSL dependency
#include <signal.h>

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

// Crypto operations (minimal - simplified without OpenSSL)
char* cursed_crypto_sha256(const char* data) {
    if (!data) return NULL;
    
    // Simple hash function (not cryptographically secure)
    // This is a placeholder for self-hosting - would need proper implementation
    size_t len = strlen(data);
    unsigned int hash = 0;
    
    for (size_t i = 0; i < len; i++) {
        hash = hash * 31 + data[i];
    }
    
    char* result = malloc(65); // 64 hex chars + null terminator
    if (result) {
        sprintf(result, "%064x", hash);
    }
    
    return result;
}

char* cursed_crypto_random_bytes(int length) {
    if (length <= 0) return NULL;
    
    char* buffer = malloc(length);
    if (buffer) {
        // Simple pseudo-random generation (not cryptographically secure)
        // This is a placeholder for self-hosting - would need proper implementation
        srand(time(NULL));
        for (int i = 0; i < length; i++) {
            buffer[i] = rand() % 256;
        }
    }
    
    return buffer;
}

// Match expression runtime support
void panic_non_exhaustive_match(void) {
    fprintf(stderr, "Error: Non-exhaustive match expression\n");
    exit(1);
}
