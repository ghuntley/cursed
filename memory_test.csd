fr fr Memory stress test program
yeet "testz"

slay memory_stress_test() {
    test_start("Memory Allocation Test")
    
    fr fr Create multiple arrays to stress memory
    sus arrays [][]normie = []
    
    bestie i := 0; i < 100; i = i + 1 {
        sus array []normie = []
        bestie j := 0; j < 1000; j = j + 1 {
            array.push(j * i)
        }
        arrays.push(array)
    }
    
    assert_eq_int(arrays.len(), 100)
    
    fr fr Large string operations
    sus large_string tea = ""
    bestie i := 0; i < 100; i = i + 1 {
        large_string = large_string + "test_string_" + i + "_"
    }
    
    assert_true(large_string.len() > 1000)
    
    print_test_summary()
}

memory_stress_test()
