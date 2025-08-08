#include <stdio.h>
#include <string.h>

// Demonstration of the pattern matching fix
// Shows the difference between buggy and fixed behavior

typedef struct {
    int value;
    char* name;
} Variable;

void buggy_pattern_matching(Variable match_var, int condition_result) {
    printf("=== BUGGY BEHAVIOR (using condition_result=%d) ===\n", condition_result);
    printf("Matching against condition_result instead of actual value\n");
    
    // This represents the buggy code that uses condition_result
    if (condition_result == 1) printf("Branch 1 executes\n");
    if (condition_result == 2) printf("Branch 2 executes\n"); 
    if (condition_result == 5) printf("Branch 5 executes\n");
    printf("Wildcard executes\n");
    printf("Result: ALL branches execute because condition_result=1 (true)\n\n");
}

void fixed_pattern_matching(Variable match_var) {
    printf("=== FIXED BEHAVIOR (using match_var.value=%d) ===\n", match_var.value);
    printf("Correctly matching against actual value\n");
    
    // This represents the fixed code that uses the actual match value
    if (match_var.value == 1) {
        printf("Branch 1 executes\n");
        return; // First match wins
    }
    if (match_var.value == 2) {
        printf("Branch 2 executes\n"); 
        return;
    }
    if (match_var.value == 5) {
        printf("Branch 5 executes\n");
        return;
    }
    printf("Wildcard executes\n");
}

int main() {
    Variable test_var = {5, "x"};
    int condition_result = 1; // This is what the buggy code incorrectly used
    
    printf("Testing pattern matching: ready (%s) where %s = %d\n\n", 
           test_var.name, test_var.name, test_var.value);
    
    buggy_pattern_matching(test_var, condition_result);
    fixed_pattern_matching(test_var);
    
    printf("=== CONCLUSION ===\n");
    printf("The fix changes the code from using condition_result to using the actual match value.\n");
    printf("This ensures only the matching branch executes, not all branches.\n");
    
    return 0;
}
