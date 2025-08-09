// Memory lifecycle test 1
sus outer1 drip = 1

ready (based) {
    sus inner1 tea = "scoped_1"
    sus calc1 drip = outer1 * 2
    vibez.spill("Scoped 1:", inner1, calc1)
}

slay memory_func_1() tea {
    sus local tea = "local_1"
    damn local + "_processed"
}

sus result tea = memory_func_1()
vibez.spill("Memory 1:", result)
