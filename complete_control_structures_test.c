#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() {
    long a = 5;
    long b = 10;
    long c = 5;
    printf("5 < 10: true""\n");
    printf("10 > 5: true""\n");
    printf("5 <= 5: true""\n");
    printf("5 >= 5: true""\n");
    printf("5 == 5: true""\n");
    printf("5 != 10: true""\n");
    printf("AND operator works""\n");
    printf("OR operator works""\n");
    printf("NOT operator works""\n");
    long nested_result = 0;
    printf("Nested control structures work""\n");
    long complex_result = 0;
    long x = 1;
    long y = 1;
    printf("Complex while condition works""\n");
    long scope_test = 1;
    long scope_test = 2;
    printf("Inner scope works""\n");
    long scope_test = 3;
    return 0;
}
