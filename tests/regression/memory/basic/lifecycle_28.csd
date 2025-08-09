// Memory lifecycle test 28
sus outer28 drip = 28

ready (based) {
    sus inner28 tea = "scoped_28"
    sus calc28 drip = outer28 * 29
    vibez.spill("Scoped 28:", inner28, calc28)
}

slay memory_func_28() tea {
    sus local tea = "local_28"
    damn local + "_processed"
}

sus result tea = memory_func_28()
vibez.spill("Memory 28:", result)
