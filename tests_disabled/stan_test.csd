vibe main;

fr fr Simple test of goroutine functionality
slay hello() {
    puts(100);
}

slay main() {
    fr fr Launch a goroutine that prints 100
    stan hello();
    
    fr fr Print from the main routine
    puts(42);
    
    fr fr Add a small delay to ensure goroutine executes
    sus i normie = 0;
    periodt i < 1000000 {
        i = i + 1;
    }
}