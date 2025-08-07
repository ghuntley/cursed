#include <stdio.h>
#include <stdlib.h>

// Test program to validate DWARF debug information generation
// This is the C equivalent of our CURSED debug test

void debug_test_function(int param1, const char* param2) {
    printf("Function called with param1=%d, param2=%s\n", param1, param2);
    
    // Local variables for debugging
    long long drip_value = 42;
    int normie_value = 123;
    double meal_value = 3.14159;
    char tea_value[] = "Hello Debug!";
    int lit_value = 1; // boolean true
    
    printf("drip_value: %lld\n", drip_value);
    printf("normie_value: %d\n", normie_value);
    printf("meal_value: %f\n", meal_value);
    printf("tea_value: %s\n", tea_value);
    printf("lit_value: %s\n", lit_value ? "based" : "cringe");
    
    // Nested scope for testing
    {
        int scoped_var = 999;
        printf("scoped_var: %d\n", scoped_var);
    }
}

int main() {
    printf("CURSED Debug Information Test\n");
    printf("This tests GDB/LLDB compatibility\n");
    
    debug_test_function(42, "test_string");
    
    return 0;
}
