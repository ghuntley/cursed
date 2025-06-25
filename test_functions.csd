// Test function declarations and control flow
facts pi = 3.14159;
facts max_users = 100;

slay calculate_area(radius) {
    facts area = pi * radius * radius;
    return area;
}

slay main() {
    facts radius = 5;
    facts area = calculate_area(radius);
    
    if (area > 50) {
        facts message = "Large area";
    } else {
        facts message = "Small area";
    }
}
