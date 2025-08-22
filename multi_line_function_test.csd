fr fr Test multi-line function parsing

slay simple_add(a drip, b drip) drip {
    damn a + b
}

slay multi_line_function(
    param1 drip,
    param2 tea
) drip {
    sus local_var drip = param1 + 5
    damn local_var
}

sus result1 drip = simple_add(3, 4)
sus result2 drip = multi_line_function(10, "test")

vibez.spill("Simple add result:", result1)
vibez.spill("Multi-line function result:", result2)
