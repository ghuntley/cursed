fr fr Test goroutine runtime integration
slay main() {
    fr fr Initialize scheduler
    vibez.spill("Starting goroutine test");
    
    fr fr Test simple goroutine spawn with runtime call
    stan worker_function();
    
    fr fr Test yield
    yolo 0;
}

slay worker_function() {
    vibez.spill("Worker started");
    yolo 1;
}
