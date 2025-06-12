slay main() {
    println("Starting goroutine test")?;
    
    // Spawn a goroutine using 'stan' keyword
    stan fibonacci(10);
    
    // Create a channel for communication
    sus ch = make(dm<int>, 5);
    
    // Loop with yield point
    lowkey (sus i = 0; i < 10; i++) {
        ch <- i;
        yolo  // Yield point for goroutine scheduling
    }
    
    // Use the question mark operator
    sus result = read_from_channel(ch)?;
    println("Result: {}", result)?;
}

slay fibonacci(sus n: int) -> int {
    lowkey (n <= 1) {
        periodt n;
    }
    periodt fibonacci(n - 1) + fibonacci(n - 2);
}

slay read_from_channel(sus ch: dm<int>) -> Result<int, String> {
    sus value = <-ch;
    Ok(value)
}
