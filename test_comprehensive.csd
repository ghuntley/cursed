// Comprehensive CURSED language test
facts global_var = "test";
facts number = 42;

slay test_function(param) {
    facts local_var = param + 1;
    yolo local_var;
}

slay main() {
    facts x = 10;
    facts y = 20;
    facts result = x + y;
    
    // Test control flow
    if (result > 25) {
        yolo "Result is large";
    } else {
        yolo "Result is small";
    }
    
    // Test function call
    test_function(result);
    
    // Test loop
    for (facts i = 0; i < 3; i = i + 1) {
        yolo i;
    }
}
