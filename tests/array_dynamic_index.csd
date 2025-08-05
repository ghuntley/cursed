vibe array_bounds_test;

slay test_dynamic_index(thicc idx) {
    sus arr normie = [100, 200, 300, 400, 500];
    if idx >= 0 && idx < 5 {
        puts(arr[idx]);
    } else {
        puts(-1); // Indicate out of bounds without crashing
    }
}

slay main() {
    // Test valid indices
    test_dynamic_index(0);
    test_dynamic_index(1);
    test_dynamic_index(2);
    test_dynamic_index(3);
    test_dynamic_index(4);
    
    // Test invalid indices
    test_dynamic_index(-1);
    test_dynamic_index(5);
    
    damn 0;
}
