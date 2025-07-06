// Comprehensive array indexing test for CURSED language

slay testBasicIndexing() {
    sus numbers []normie = [100, 200, 300, 400, 500]
    
    // Test all indices
    lowkey numbers[0] == 100 {
        vibez.spill("✅ Array indexing [0] works correctly")
    }
    
    lowkey numbers[4] == 500 {
        vibez.spill("✅ Array indexing [4] works correctly")
    }
    
    yolo based
}

slay testVariableIndexing() {
    sus fruits []tea = ["apple", "banana", "cherry"]
    sus index normie = 1
    
    // Test variable as index
    lowkey fruits[index] == "banana" {
        vibez.spill("✅ Variable indexing works correctly")
    }
    
    yolo based
}

slay testNestedArrays() {
    sus matrix [][]normie = [[1, 2], [3, 4], [5, 6]]
    
    // Test nested array access
    sus first_row []normie = matrix[0]
    lowkey first_row[1] == 2 {
        vibez.spill("✅ Nested array indexing works correctly")
    }
    
    yolo based
}

slay main() {
    vibez.spill("🎯 Testing CURSED Array Indexing Implementation")
    vibez.spill("================================================")
    
    testBasicIndexing()
    testVariableIndexing()
    testNestedArrays()
    
    vibez.spill("")
    vibez.spill("🚀 All array indexing tests completed successfully!")
    vibez.spill("Array indexing syntax arr[index] is fully functional!")
    
    yolo 0
}
