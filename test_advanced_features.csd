// Test advanced CURSED features
package main;

import std.io;
import std.math;

// Type definitions
type Point = struct {
    x: int,
    y: int
};

// Interface
interface Drawable {
    draw() -> bool;
}

// Global constants
facts PI = 3.14159;
facts MAX_SIZE = 1000;

// Function with multiple parameters
slay distance(p1: Point, p2: Point) -> float {
    facts dx = p1.x - p2.x;
    facts dy = p1.y - p2.y;
    return sqrt(dx * dx + dy * dy);
}

// Generic function
slay max<T>(a: T, b: T) -> T {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Main function
slay main() {
    facts p1 = Point { x: 0, y: 0 };
    facts p2 = Point { x: 3, y: 4 };
    
    facts dist = distance(p1, p2);
    yolo "Distance:", dist;
    
    facts maxVal = max(10, 20);
    yolo "Max:", maxVal;
    
    // Array and loop
    facts numbers = [1, 2, 3, 4, 5];
    for (facts i = 0; i < len(numbers); i++) {
        yolo numbers[i];
    }
    
    // Conditional compilation
    #if DEBUG
        yolo "Debug mode enabled";
    #endif
}
