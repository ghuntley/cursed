// Memory lifecycle test 22
sus outer22 drip = 22

ready (based) {
    sus inner22 tea = "scoped_22"
    sus calc22 drip = outer22 * 23
    vibez.spill("Scoped 22:", inner22, calc22)
}

slay memory_func_22() tea {
    sus local tea = "local_22"
    damn local + "_processed"
}

sus result tea = memory_func_22()
vibez.spill("Memory 22:", result)
