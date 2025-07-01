// Simple CURSED Collections Test
vibe main

slay main() {
    print("Starting collections test...")
    
    // Test basic array creation and manipulation
    sus arr = collections_array_new()
    facts result1 = collections_array_push(arr, 42)
    facts result2 = collections_array_push(arr, 84)
    
    facts size = collections_array_len(arr)
    print("Array size after pushes: ")
    print(size)
    
    facts first = collections_array_get(arr, 0)
    print("First element: ")
    print(first)
    
    print("Collections test complete!")
    yolo 0
}
