#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() {
    printf("Testing advanced compilation features""\n");
    long result = add_numbers(10, 32);
    printf("10 + 32 =""\n");
    printf("%ld\n", result);
    int flag = 1;
    char* name = "CURSED compiler";
    printf("Compiler name:""\n");
    printf("%s\n", name);
    printf("Flag value:""\n");
    printf("%s\n", flag ? "based" : "cringe");
    return 0;
}
