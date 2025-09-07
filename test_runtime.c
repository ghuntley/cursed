#include <stdio.h>
void cursed_runtime_spill_string(const char* str) { printf("%s\n", str); }
int main() { cursed_runtime_spill_string("Test string"); return 0; }
