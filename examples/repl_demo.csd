fr fr CURSED REPL Demo Program
fr fr This file demonstrates the features available in the CURSED REPL
fr fr Run this in the REPL using: :load examples/repl_demo.csd

fr fr Variable declarations with Gen Z slang
facts age = 25
sus score = 100.5
facts name = "Taylor"
sus is_valid = based

fr fr Function declarations
slay greet(name) {
    println("Yo, what's good " + name + "! 🔥")
}

slay calculate_energy(base, multiplier) {
    facts energy = base * multiplier
    periodt energy
}

slay main_character() {
    // Call functions
    greet(name)
    
    // Local variables
    facts energy_level = calculate_energy(score, 2.0)
    println("Energy level is straight fire: " + energy_level)
    
    // Control flow with CURSED syntax
    lowkey (age > 18) {
        println("Old enough to be fire! 🚀")
    } bestie {
        println("Still growing that energy! 💫")
    }
    
    // Loop with CURSED syntax
    lowkey (sus i = 0; i < 3; i++) {
        println("Loop iteration: " + i)
        yolo // yield point for cooperative scheduling
    }
    
    println("That's the demo, no cap! ✨")
}

fr fr Run the main function
main_character()
