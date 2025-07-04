fr fr Comprehensive test of function parameter types and return type inference

fr fr Test explicit parameter and return types
slay explicit_typed(x normie, name tea) tea {
    yolo name + " has value " + x;
}

fr fr Test return type inference from literal
slay infer_number() {
    yolo 42;
}

fr fr Test return type inference from string literal
slay infer_string() {
    yolo "Hello world";
}

fr fr Test return type inference from boolean literal
slay infer_boolean() {
    yolo based;
}

fr fr Test return type inference from expression
slay infer_math() {
    yolo 5 + 10;
}

fr fr Test mixed types
slay process_data(count normie, message tea, active vibes) {
    lowkey active {
        yolo message + " count: " + count;
    }
    yolo "inactive";
}

slay main() {
    sus result1 tea = explicit_typed(100, "Score");
    sus result2 = infer_number();
    sus result3 = infer_string();  
    sus result4 = infer_boolean();
    sus result5 = infer_math();
    sus result6 = process_data(5, "Items", based);
    
    vibez.spill(result1);
    vibez.spill(result6);
    
    yolo result2;
}
