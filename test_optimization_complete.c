#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() {
    long global_var = 100;
    long result = 0;
    long i = 0;
    printf("Testing optimization with complex calculations...""\n");
    long unused_var = 42;
    long sum = 0;
    long j = 0;
    long calc_result = expensive_calculation(5);
    printf("Sum: ""\n");
    printf("%ld\n", sum);
    printf("Calculation result: ""\n");
    printf("%ld\n", calc_result);
    long const_a = 5;
    long const_b = 10;
    long const_result = const_a * const_b;
    printf("Constant result: ""\n");
    printf("%ld\n", const_result);
    return 0;
}
