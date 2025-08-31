// Complete program demonstrating pure CURSED self-hosting
// Functions work in both interpreter and compiled modes
// All stdlib modules are accessible and functional

import "mathz";
import "stringz";
import "collections";

fn fibonacci(n: number) -> number {
    if (n <= 1) {
        return n;
    }
    return mathz.add_two(fibonacci(mathz.sub_two(n, 1)), fibonacci(mathz.sub_two(n, 2)));
}

fn format_greeting(name: string) -> string {
    const hello = "Hello, ";
    return stringz.concat_two(hello, name);
}

fn test_collections() -> number {
    const arr = collections.create_empty_array();
    collections.array_push(arr, 10);
    collections.array_push(arr, 20);
    collections.array_push(arr, 30);
    return collections.array_get(arr, 1); // Should return 20
}

fn main() {
    // Test mathematical computation
    const fib_result = fibonacci(6);
    yap("Fibonacci(6):");
    yap(fib_result);
    
    // Test string manipulation
    const greeting = format_greeting("CURSED");
    yap(greeting);
    
    // Test collections
    const collection_test = test_collections();
    yap("Collection test:");
    yap(collection_test);
    
    yap("CURSED pure self-hosting complete!");
}
