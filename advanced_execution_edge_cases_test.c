#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() {
    char* empty = "";
    printf("%s\n", empty);
    char* unicode = "🚀 CURSED";
    printf("%s\n", unicode);
    long big = 999999;
    printf("%ld\n", big);
    long result = quadruple(5);
    printf("Nested function result:""\n");
    printf("%ld\n", result);
    int truth = 1;
    int lie = 0;
    printf("%s\n", truth ? "based" : "cringe");
    printf("%s\n", lie ? "based" : "cringe");
    printf("Array test:""\n");
    return 0;
}
