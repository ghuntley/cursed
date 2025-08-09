// Memory lifecycle test 12
sus outer12 drip = 12

ready (based) {
    sus inner12 tea = "scoped_12"
    sus calc12 drip = outer12 * 13
    vibez.spill("Scoped 12:", inner12, calc12)
}

slay memory_func_12() tea {
    sus local tea = "local_12"
    damn local + "_processed"
}

sus result tea = memory_func_12()
vibez.spill("Memory 12:", result)
