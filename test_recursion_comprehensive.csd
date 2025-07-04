slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        yolo n;
    }
    yolo fibonacci(n - 1) + fibonacci(n - 2);
}

slay countdown(n normie) normie {
    vibez.spill("Counting down:");
    vibez.spill(n);
    lowkey n <= 0 {
        yolo n;
    }
    yolo countdown(n - 1);
}

slay power(base normie, exp normie) normie {
    lowkey exp <= 0 {
        yolo 1;
    }
    yolo base * power(base, exp - 1);
}

slay main() {
    vibez.spill("=== Recursion Tests ===");
    
    vibez.spill("Fibonacci(7):");
    yolo fibonacci(7);
    
    vibez.spill("Countdown from 5:");
    yolo countdown(5);
    
    vibez.spill("Power(2, 4):");
    yolo power(2, 4);
    
    vibez.spill("=== All tests completed! ===");
}
