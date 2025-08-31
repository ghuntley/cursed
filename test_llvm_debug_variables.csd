// Test file to debug LLVM variable scoping and access
// This tests global vs local variables in compiled mode

sus global_var drip = 42

sus test_function(param drip) -> i32 {
    sus local_var drip = param + 10
    yap("In function - param:")
    yap(param)
    yap("In function - local_var:")
    yap(local_var)
    yap("In function - global_var:")
    yap(global_var)
    return local_var
}

sus main() -> i32 {
    yap("Testing variable scoping:")
    yap("global_var should be 42")
    yap(global_var)
    
    sus local_to_main drip = 100
    yap("local_to_main should be 100")  
    yap(local_to_main)
    
    sus function_result drip = test_function(25)
    yap("test_function(25) should return 35")
    yap(function_result)
    
    return 0
}
