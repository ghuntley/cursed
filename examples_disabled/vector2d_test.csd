// Test Vector2D user-defined type methods

// Define a Vector2D with x=3 and y=4
Vector2D{x: 3, y: 4}

// Calculate the length (should be 5)
Vector2D.length()

// Add 2 to both coordinates
// Note: arguments need to be strings for now
Vector2D.add("2", "2")

// Get the string representation
Vector2D.toString()

// Now use vibez.spill to output the results
vibez.spill("Vector calculation complete!")