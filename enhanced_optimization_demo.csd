# Enhanced LLVM Optimization Demo
# This test demonstrates the enhanced optimization pipeline with improved inlining

yeet "testz"

# Test functions for inlining optimization
slay small_function(x drip) drip {
    damn x + 1
}

slay medium_function(a drip, b drip) drip {
    sus result drip = small_function(a)
    result = result + small_function(b)
    damn result
}

slay complex_function(x drip) drip {
    sus total drip = 0
    aura i drip = 0; i < x; i++ {
        total = total + medium_function(i, i + 1)
    }
    damn total
}

# Interface for testing interface inlining
interface Calculator {
    calculate(x drip) drip
}

structure SimpleCalculator {
    multiplier drip
}

impl Calculator aura SimpleCalculator {
    slay calculate(x drip) drip {
        damn x * sick.multiplier
    }
}

# Generic function for testing generics inlining
slay generic_add<T>(a T, b T) T {
    damn a + b
}

# Main function with various optimization opportunities
slay main() {
    vibez.spill("Enhanced Optimization Demo")
    
    # Test basic function inlining
    sus result1 drip = small_function(5)
    vibez.spill("Small function result: " + result1.tea())
    
    # Test medium function inlining (should inline small_function calls)
    sus result2 drip = medium_function(10, 20)
    vibez.spill("Medium function result: " + result2.tea())
    
    # Test complex function with loop optimization
    sus result3 drip = complex_function(5)
    vibez.spill("Complex function result: " + result3.tea())
    
    # Test interface optimization
    sus calc SimpleCalculator = SimpleCalculator{multiplier: 3}
    sus interface_calc Calculator = calc
    sus result4 drip = interface_calc.calculate(7)
    vibez.spill("Interface calculation result: " + result4.tea())
    
    # Test generic function inlining
    sus result5 drip = generic_add(15, 25)
    vibez.spill("Generic add result: " + result5.tea())
    
    # Test with various optimization scenarios
    sus i drip = 0
    sus total drip = 0
    aura i < 100 {
        total = total + small_function(i)
        i = i + 1
    }
    vibez.spill("Loop optimization result: " + total.tea())
    
    vibez.spill("Enhanced optimization demo complete!")
}
