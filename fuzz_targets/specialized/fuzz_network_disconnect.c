// CURSED Network Fuzz Target Template
// Targets: disconnect in src-zig/windows_async_network.zig:549

const std = @import("std");
const testing = std.testing;
const c = @cImport({
    @cInclude("stdint.h");
    @cInclude("stdlib.h");
    @cInclude("string.h");
});
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#define MAX_PACKET_SIZE 65535

// Zig-specific setup
extern fn malloc(size: usize) ?*anyopaque;
extern fn free(ptr: ?*anyopaque) void;

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0 || size > MAX_PACKET_SIZE) return 0;
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test packet parsing
    // Test disconnect with network data
    // Example: disconnect(data, size);
    // Example: disconnect(sockfd, data, size);
    
    // Test with socket operations (mock)
    test_socket_operations(data, size);
    
    // Zig GPA cleanup handled by defer
    return 0;
}

void test_socket_operations(const uint8_t *data, size_t size) {
    // Create mock socket operations
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return;
    
    // Test with fuzzed network data
    // disconnect(sockfd, data, size);
    
    close(sockfd);
}


// Additional test functions for disconnect
void test_edge_cases(const uint8_t *data, size_t size) {
    // Test with edge cases specific to network
}

void test_error_conditions(const uint8_t *data, size_t size) {
    // Test error handling paths
}

