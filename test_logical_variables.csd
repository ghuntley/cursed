slay main() {
    let a = true;
    let b = false;
    
    if (a && b) {
        vibez.spill("a && b should not print");
    }
    
    if (a || b) {
        vibez.spill("a || b works with variables!");
    }
    
    if (b || a) {
        vibez.spill("b || a works with variables!");
    }
}
