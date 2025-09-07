#include <stdio.h>
void cursed_runtime_spill_string(const char* str) { printf("%s\n", str); fflush(stdout); }
int main() { cursed_runtime_spill_string("Direct test"); return 0; }
