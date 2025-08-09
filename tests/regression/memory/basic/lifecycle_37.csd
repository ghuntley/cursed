// Memory lifecycle test 37
sus outer37 drip = 37

ready (based) {
    sus inner37 tea = "scoped_37"
    sus calc37 drip = outer37 * 38
    vibez.spill("Scoped 37:", inner37, calc37)
}

slay memory_func_37() tea {
    sus local tea = "local_37"
    damn local + "_processed"
}

sus result tea = memory_func_37()
vibez.spill("Memory 37:", result)
