// Comprehensive CURSED Language Test
// Testing multiple features including functions, arrays, control flow

fn main() {
    let result = fibonacci_iterative(10);
    println("Fibonacci(10) iterative = {}", result);
    
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum_result = array_sum(nums);
    println("Sum of array: {}", sum_result);
    
    test_control_flow();
    
    println("CURSED language comprehensive test completed successfully!");
}

fn fibonacci_iterative(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    
    let mut a = 0;
    let mut b = 1;
    let mut i = 2;
    
    while i <= n {
        let temp = a + b;
        a = b;
        b = temp;
        i = i + 1;
    }
    
    return b;
}

fn array_sum(arr: [i32; 10]) -> i32 {
    let mut total = 0;
    let mut index = 0;
    
    while index < 10 {
        total = total + arr[index];
        index = index + 1;
    }
    
    return total;
}

fn test_control_flow() {
    let test_value = 5;
    
    if test_value > 3 {
        println("Test value {} is greater than 3", test_value);
    } else {
        println("Test value {} is not greater than 3", test_value);
    }
    
    let mut counter = 0;
    while counter < 3 {
        println("Counter: {}", counter);
        counter = counter + 1;
    }
}
