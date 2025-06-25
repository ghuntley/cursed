vibe main_program

fr fr Import utilities from other packages
fr fr import "package1"
fr fr import "package2"

slay main() {
    vibez.spill("Multi-package CURSED program")
    
    fr fr Test mathematical operations
    fr fr sus sum = package1.add(10, 5)
    fr fr sus product = package1.multiply(4, 7)
    fr fr sus power_result = package1.power(2, 3)
    
    fr fr vibez.spill("10 + 5 =", sum)
    fr fr vibez.spill("4 * 7 =", product) 
    fr fr vibez.spill("2^3 =", power_result)
    
    fr fr Test array operations
    sus numbers = []normie{3, 1, 4, 1, 5, 9, 2, 6}
    fr fr sus reversed = package2.reverse_array(numbers)
    fr fr sus total = package2.sum_array(numbers)
    fr fr sus maximum = package2.find_max(numbers)
    
    fr fr vibez.spill("Original array:", numbers)
    fr fr vibez.spill("Reversed array:", reversed)
    fr fr vibez.spill("Sum:", total)
    fr fr vibez.spill("Maximum:", maximum)
    
    fr fr Test conditional logic
    fr fr issa package1.is_even(total) {
    fr fr     vibez.spill("Sum is even")
    fr fr } else {
    fr fr     vibez.spill("Sum is odd")
    fr fr }
    
    vibez.spill("Program completed successfully!")
}
