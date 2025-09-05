#include <stdio.h>

int main() {
    double val1 = 2147483648.0;
    double val2 = 3000000000.0;
    
    printf("C printf %%g format:\n");
    printf("2147483648.0 = %g\n", val1);
    printf("3000000000.0 = %g\n", val2);
    
    return 0;
}
