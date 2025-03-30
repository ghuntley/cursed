fr fr Complex test demonstrating multiple features of CURSED

fr fr Variable declarations and assignments
sus message = "Hello, CURSED!";
sus x = 10;
sus y = 20;
sus result = 0;

fr fr Conditional logic
lowkey (x < y) {
    println("x is less than y");
    result = x + y;
} highkey {
    println("x is greater than or equal to y");
    result = x - y;
}

fr fr Loop with a counter
sus counter = 0;
periodt (counter < 5) {
    println("Counter value:");
    puts(counter);
    counter = counter + 1;
}

fr fr Final output
println("Final result:");
puts(result);
println("Test completed successfully!");
