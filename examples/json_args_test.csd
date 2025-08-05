vibe main

yeet "vibez"
yeet "mathz"

slay main() {
    // Test with string arguments (already working)
    vibez.spill("Testing non-string arguments");
    
    // Test with numeric arguments (not fully supported yet, but shows the concept)
    sus pi := 3.14159;
    sus radius := 5;
    sus area := mathz.CalculateArea(pi, radius); // mathz.CalculateArea would take JSON number arguments
    
    // For demonstration purposes, we'll print the result
    vibez.spill("Circle area with radius 5: " + area);
    
    // Test with boolean arguments
    sus is_enabled := based;
    sus result := mathz.ConditionalCalculation(is_enabled, 10); // Takes a boolean and a number
    
    vibez.spill("Conditional calculation result: " + result);
    
    damn 0;
}