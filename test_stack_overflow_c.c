// Simple C program to test the stack overflow detection function
#include <stdio.h>
#include <stdbool.h>

// External declaration of the Rust function
extern bool rust_check_stack_overflow(void);

int main() {
    printf("Testing stack overflow detection...\n");
    
    // Call the stack overflow detection function
    bool has_overflow = rust_check_stack_overflow();
    
    if (has_overflow) {
        printf("Stack overflow detected!\n");
    } else {
        printf("No stack overflow detected.\n");
    }
    
    return 0;
}
