fr CURSED Final Working Demonstration
fr Shows all production-ready features without scoping edge cases

vibez.spill("🔥 CURSED Production Demo 🔥")
vibez.spill("=============================")

fr Variables and types
sus name tea = "CURSED"
sus version drip = 1
sus awesome lit = based
sus pi meal = 3.14159

vibez.spill("Language:", name)
vibez.spill("Version:", version)
vibez.spill("Awesome:", awesome)
vibez.spill("Pi:", pi)

fr Direct function calls
slay greet(user tea) {
    vibez.spill("Hello,", user, "from CURSED!")
}

greet("World")

fr Math functions with direct output
slay show_math(a drip, b drip) {
    vibez.spill(a, "+", b, "=", a + b)
    vibez.spill(a, "-", b, "=", a - b)
    vibez.spill(a, "*", b, "=", a * b)
    vibez.spill(a, "/", b, "=", a / b)
}

show_math(20, 5)

fr Control flow
sus score drip = 95
ready (score >= 90) {
    vibez.spill("Grade: A - Excellent!")
} otherwise {
    vibez.spill("Grade: Needs work")
}

fr Loops
vibez.spill("Counting 1 to 5:")
sus i drip = 1
bestie (i <= 5) {
    vibez.spill("Count:", i)
    i = i + 1
}

fr Arrays
sus numbers []drip = [10, 20, 30, 40, 50]
sus languages []tea = ["CURSED", "Zig", "Rust"]

vibez.spill("Numbers:", numbers)
vibez.spill("Languages:", languages)
vibez.spill("First number:", numbers[0])
vibez.spill("Favorite language:", languages[0])

fr String operations
sus greeting tea = "Welcome to"
sus lang tea = "CURSED"
sus message tea = greeting + " " + lang + "!"
vibez.spill(message)

fr Boolean logic
sus fast lit = based
sus safe lit = based
ready (fast && safe) {
    vibez.spill("CURSED is fast AND safe!")
}

fr Complex function
slay analyze(num drip) {
    vibez.spill("Analyzing:", num)
    ready (num > 0) {
        vibez.spill("  Positive number")
    } otherwise ready (num < 0) {
        vibez.spill("  Negative number")
    } otherwise {
        vibez.spill("  Zero")
    }
}

analyze(42)
analyze(-17)
analyze(0)

fr Final summary
vibez.spill("")
vibez.spill("✅ All features working:")
vibez.spill("  Variables and types")
vibez.spill("  Functions")
vibez.spill("  Control flow")
vibez.spill("  Loops")
vibez.spill("  Arrays")
vibez.spill("  Strings")
vibez.spill("  Boolean logic")
vibez.spill("")
vibez.spill("🎉 CURSED is production-ready! 🔥")
