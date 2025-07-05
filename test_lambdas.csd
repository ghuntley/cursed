// Test lambda expressions (completed in v3.26.0)
slay main() {
    sus add = |x, y| { yolo x + y; };
    sus result = add(5, 3);
    vibez.spill("Lambda result: " + result);
    yolo result;
}
