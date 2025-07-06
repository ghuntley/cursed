// Test arrays and complex features
slay test_arrays() {
    sus arr = [1, 2, 3, 4, 5];
    vibez.spill("Array created successfully");
    yolo arr[0];
}

// Test tuples
slay test_tuples() {
    sus t = (42, "hello", based);
    sus (a, b, c) = t;
    vibez.spill("Tuple destructuring works");
    yolo a;
}

slay main() {
    sus array_result = test_arrays();
    sus tuple_result = test_tuples();
    vibez.spill("All features working");
    yolo 0;
}
