vibe main;

yeet "mathutils"
yeet "stringutils"

slay main_character() {
    vibez.spill("=== CURSED Separate Compilation Demo ===")
    
    fr fr Test math utilities
    vibez.spill("--- Math Utils Test ---")
    sus a = 5
    sus b = 3
    
    sus sum = mathutils.add(a, b)
    sus product = mathutils.multiply(a, b)
    sus doubled = mathutils.double(a)
    sus tripled = mathutils.triple(b)
    
    vibez.spill("5 + 3 =")
    vibez.spill(sum)
    vibez.spill("5 * 3 =")
    vibez.spill(product)
    vibez.spill("5 * 2 =")
    vibez.spill(doubled)
    vibez.spill("3 * 3 =")
    vibez.spill(tripled)
    
    fr fr Test string utilities  
    vibez.spill("--- String Utils Test ---")
    sus str1 = "World"
    sus str2 = "Alice"
    
    sus combined = stringutils.concat(str1, str2)
    sus greeting = stringutils.greet(str1)
    sus farewell = stringutils.goodbye(str2)
    
    vibez.spill("Combined:")
    vibez.spill(combined)
    vibez.spill("Greeting:")
    vibez.spill(greeting)
    vibez.spill("Farewell:")
    vibez.spill(farewell)
    
    fr fr Test cross-package functionality
    vibez.spill("--- Cross-Package Test ---")
    sus number = 7
    sus doubled_num = mathutils.double(number)
    sus msg = stringutils.greet("Math")
    
    vibez.spill("Doubled 7:")
    vibez.spill(doubled_num)
    vibez.spill("Message:")
    vibez.spill(msg)
    
    vibez.spill("=== Demo Complete ===")
}
