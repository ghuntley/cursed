#include <stdio.h>
#include <stdint.h>

// Declare the runtime function
extern int32_t mathz_abs_normie(int32_t x);

int main() {
    int32_t test1 = 42;
    int32_t test2 = -42;
    int32_t test3 = 0;
    
    int32_t result1 = mathz_abs_normie(test1);
    int32_t result2 = mathz_abs_normie(test2);
    int32_t result3 = mathz_abs_normie(test3);
    
    printf("abs(42) = %d\n", result1);
    printf("abs(-42) = %d\n", result2); 
    printf("abs(0) = %d\n", result3);
    
    return 0;
}
