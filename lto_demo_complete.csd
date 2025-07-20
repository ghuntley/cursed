fr fr LTO optimization demo program

slay simple_add(a drip, b drip) drip {
    damn a + b
}

slay complex_calculation(x drip) drip {
    sus result drip = simple_add(x, 42)
    sus temp drip = result * 2
    damn temp + simple_add(5, 10)
}

slay constant_heavy_function() drip {
    sus const1 drip = 100
    sus const2 drip = 200
    sus const3 drip = 300
    damn simple_add(const1, simple_add(const2, const3))
}

slay unused_function(param drip) drip {
    damn param * 999
}

slay main() drip {
    vibez.spill("=== CURSED LTO Optimization Demo ===")
    
    sus result1 drip = simple_add(10, 20)
    vibez.spill("Simple add result: ")
    vibez.spill(result1)
    
    sus result2 drip = complex_calculation(50)
    vibez.spill("Complex calculation result: ")
    vibez.spill(result2)
    
    sus result3 drip = constant_heavy_function()
    vibez.spill("Constant heavy function result: ")
    vibez.spill(result3)
    
    vibez.spill("=== LTO Demo Complete ===")
    damn 0
}
