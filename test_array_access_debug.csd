fr fr Test array access and variable interpolation

yeet "vibez"

slay main() {
    vibez.spill("Testing array access and interpolation")
    
    fr fr Test basic array creation and access
    sus numbers [normie] = [10, 20, 30, 40, 50]
    vibez.spill("Array created: [10, 20, 30, 40, 50]")
    
    fr fr Test direct array access
    vibez.spill("numbers[0] =")
    vibez.spill(numbers[0])
    vibez.spill("numbers[2] =")  
    vibez.spill(numbers[2])
    
    fr fr Test array access in loop
    sus i normie = 0
    periodt i < numbers.length {
        vibez.spill("Element at index")
        vibez.spill(i)
        vibez.spill("is")
        vibez.spill(numbers[i])
        i = i + 1
    }
    
    fr fr Test variable interpolation with array access (if supported)
    fr fr vibez.spillf("numbers[0] = {}", numbers[0])
    fr fr vibez.spillf("numbers[2] = {}", numbers[2])
}
