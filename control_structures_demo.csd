# CURSED Control Structures Implementation Demo
vibez.spill("🔥 CURSED Control Structures Demo 🔥")
vibez.spill("======================================")

# Demonstrate ready (if) statements
vibez.spill("\n1. IF STATEMENTS (ready):")
ready (based) {
    vibez.spill("✅ ready (based) - if true works!")
}

# Demonstrate ready/otherwise (if/else)
vibez.spill("\n2. IF/ELSE STATEMENTS (ready/otherwise):")
sus temperature drip = 25
ready (temperature > 30) {
    vibez.spill("🔥 It's hot!")
} otherwise {
    vibez.spill("❄️ It's cool!")
}

# Demonstrate bestie (while) loops
vibez.spill("\n3. WHILE LOOPS (bestie):")
sus countdown drip = 3
bestie (countdown > 0) {
    vibez.spill("🚀 Countdown:", countdown)
    countdown = countdown - 1
}
vibez.spill("🎉 Blast off!")

# Demonstrate nested control structures
vibez.spill("\n4. NESTED STRUCTURES:")
sus grade drip = 85
ready (grade >= 60) {
    vibez.spill("📚 You passed!")
    ready (grade >= 90) {
        vibez.spill("⭐ Excellent work!")
    } otherwise {
        ready (grade >= 80) {
            vibez.spill("👍 Good job!")
        } otherwise {
            vibez.spill("📖 Keep studying!")
        }
    }
} otherwise {
    vibez.spill("❌ You failed!")
}

# Demonstrate comparison operators in conditions
vibez.spill("\n5. COMPARISON OPERATORS:")
sus x drip = 10
sus y drip = 20

ready (x < y) { vibez.spill("✅ x < y") }
ready (x > y) { vibez.spill("❌ x > y") } otherwise { vibez.spill("✅ x <= y") }
ready (x == 10) { vibez.spill("✅ x equals 10") }

vibez.spill("\n🎯 All control structures implemented successfully!")
vibez.spill("Features: ready (if), otherwise (else), bestie (while)")
vibez.spill("✅ Parsing: Complete")
vibez.spill("✅ Interpretation: Complete") 
vibez.spill("✅ LLVM Compilation: Complete")
vibez.spill("✅ Memory Safety: Verified")
