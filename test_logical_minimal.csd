slay main() {
    // Direct logical operations
    if (true && true) {
        vibez.spill("true && true works!");
    }
    
    if (true || false) {
        vibez.spill("true || false works!");
    }
    
    if (false && true) {
        vibez.spill("false && true should not print");
    }
    
    if (false || false) {
        vibez.spill("false || false should not print");
    }
}
