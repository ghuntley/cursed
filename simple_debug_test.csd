slay debug_function(x drip) drip {
    sus local_var drip = x + 10
    vibez.spill("Local variable:", local_var)
    damn local_var
}

sus global_var drip = 42
sus result drip = debug_function(global_var)
vibez.spill("Final result:", result)
