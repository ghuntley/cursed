# Closures test
yeet "vibez"

# Simple closure that captures a variable
sus outer_var drip = 42

slay create_closure() slay() drip {
    damn slay() drip {
        damn outer_var * 2
    }
}

sus closure_func slay() drip = create_closure()
sus result drip = closure_func()

vibez.spill("Closure result:", result)

# Closure with parameter
slay make_adder(x drip) slay(drip) drip {
    damn slay(y drip) drip {
        damn x + y
    }
}

sus add_five slay(drip) drip = make_adder(5)
sus add_result drip = add_five(10)

vibez.spill("Closure with param result:", add_result)
