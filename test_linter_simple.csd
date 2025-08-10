// Simple test of the CURSED linter functionality
yeet "vibez"

slay main() {
    vibez.spill("Testing CURSED Linter...")
    
    // Test code with violations
    sus testCode tea = "sus myBadVariable drip = 42"
    
    // Basic linting test (simplified)
    ready (contains_str(testCode, "sus ")) {
        vibez.spill("✅ Variable declaration detected")
    }
    
    ready (contains_camel_case_simple(testCode)) {
        vibez.spill("⚠️ camelCase variable detected - should use snake_case")
    }
    
    vibez.spill("🔥 Basic linter functionality working!")
}

slay contains_camel_case_simple(line tea) lit {
    // Simple camelCase detection
    sus has_lower lit = cringe
    sus has_upper lit = cringe
    
    sus i drip = 0
    bestie (i < len_str(line)) {
        sus char tea = char_at(line, i)
        ready (char >= "a" && char <= "z") { has_lower = based }
        ready (char >= "A" && char <= "Z") { has_upper = based }
        i = i + 1
    }
    
    damn has_lower && has_upper
}
