fr fr Pointer test demo for CURSED language

slay main() {
    fr fr Create a variable
    sus x normie = 42
    
    fr fr Create a pointer to x with @
    sus ptr @normie = @x
    
    fr fr Dereference the pointer to get the value
    sus value normie = *ptr
    
    fr fr Output the results
    vibez.spill("Original value: " + x)
    vibez.spill("Value through pointer: " + value)
    
    fr fr Modify the value through the pointer
    *ptr = 100
    
    fr fr Check that x changed
    vibez.spill("Modified value: " + x)
    
    yolo 0
}
