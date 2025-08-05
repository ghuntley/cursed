slay debug_test_function(param normie) normie {
    sus local_var drip = 100
    sus name tea = "debug_test"
    
    vibez.spill("Testing debug info generation")
    vibez.spill("param = " + param.tea())
    vibez.spill("local_var = " + local_var.tea())
    vibez.spill("name = " + name)
    
    damn param + local_var.normie()
}

slay main() normie {
    sus result normie = debug_test_function(42)
    vibez.spill("Result: " + result.tea())
    damn 0
}
