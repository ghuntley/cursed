#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() {
    long local_var = x + 10;
    printf("%s\n", ""Local variable:", local_var");
    long global_var = 42;
    long result = debug_function(global_var);
    printf("%s\n", ""Final result:", result");
    return 0;
}
