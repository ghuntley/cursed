// Test CURSED JIT compilation
func main() {
    print("Testing JIT compilation");
    
    // Test basic function
    func greet(name) {
        print("Hello, " + name);
    }
    
    greet("CURSED JIT");
    
    // Test goroutine
    go func() {
        print("Running in goroutine");
    }();
    
    // Test channels
    ch := make(chan string);
    go func() {
        ch <- "Channel message";
    }();
    msg := <-ch;
    print(msg);
    
    // Test async/await
    async func async_work() {
        print("Async work starting");
        await delay(100);
        print("Async work completed");
    }
    
    await async_work();
    
    print("JIT test completed successfully!");
}
