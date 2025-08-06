#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char* message = "Hello, World!";
    long count = 42L;
    int active = 1;
    char temp_str[1024];
    sprintf(temp_str, "Test message: " "%s", message);
    printf("%s\n", temp_str);
    char temp_str[1024];
    sprintf(temp_str, "Count is: " "%ld", count);
    printf("%s\n", temp_str);
    char temp_str[1024];
    sprintf(temp_str, "Status: " "%s", active ? "based" : "cringe");
    printf("%s\n", temp_str);
    return 0;
}
