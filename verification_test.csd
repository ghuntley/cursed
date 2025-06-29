// CURSED Language Verification Test
// Testing full LLVM compilation and execution

fn main() {
    let result = calculate_fibonacci(10);
    println("Fibonacci(10) = {}", result);
    
    let test_string = "CURSED is working!";
    println("Test string: {}", test_string);
    
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_array(numbers);
    println("Sum of array: {}", sum);
}

fn calculate_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}

fn sum_array(arr: [i32; 5]) -> i32 {
    let mut total = 0;
    for i in arr {
        total = total + i;
    }
    return total;
}
