slay main() {
    select {
        ready value: 
            vibez.spill("Ready case")
        basic:
            vibez.spill("Default case")
    }
}
