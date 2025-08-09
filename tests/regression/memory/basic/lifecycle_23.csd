// Memory lifecycle test 23
sus outer23 drip = 23

ready (based) {
    sus inner23 tea = "scoped_23"
    sus calc23 drip = outer23 * 24
    vibez.spill("Scoped 23:", inner23, calc23)
}

slay memory_func_23() tea {
    sus local tea = "local_23"
    damn local + "_processed"
}

sus result tea = memory_func_23()
vibez.spill("Memory 23:", result)
