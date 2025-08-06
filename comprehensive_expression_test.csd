slay test_all_expression_types() {
    // Test basic literals
    sus int_val drip = 42
    sus float_val meal = 3.14
    sus str_val tea = "hello"
    sus bool_val lit = based
    sus char_val sip = 'A'
    
    // Test binary operations
    sus sum = int_val + 10
    sus diff = int_val - 5
    sus product = int_val * 2
    sus quotient = int_val / 2
    sus remainder = int_val % 3
    
    // Test comparison operations
    sus eq = int_val == 42
    sus ne = int_val != 0
    sus lt = int_val < 100
    sus le = int_val <= 42
    sus gt = int_val > 0
    sus ge = int_val >= 42
    
    // Test logical operations
    sus and_result = bool_val && based
    sus or_result = bool_val || cringe
    
    // Test arrays
    sus arr = [1, 2, 3, 4, 5]
    sus first = arr[0]
    sus slice = arr[1:3]
    
    // Test variable increment/decrement
    int_val++
    int_val--
    
    // Test function calls
    vibez.spill("All expression types working!")
    vibez.spillf("Integer: {d}, Float: {f}, String: {s}", int_val, float_val, str_val)
    
    // Test type assertions
    sus asserted = int_val.(thicc)
    
    // Test member access
    sus arr_len = arr.len
    
    damn sum
}
