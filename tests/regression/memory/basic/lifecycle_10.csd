// Memory lifecycle test 10
sus outer10 drip = 10

ready (based) {
    sus inner10 tea = "scoped_10"
    sus calc10 drip = outer10 * 11
    vibez.spill("Scoped 10:", inner10, calc10)
}

slay memory_func_10() tea {
    sus local tea = "local_10"
    damn local + "_processed"
}

sus result tea = memory_func_10()
vibez.spill("Memory 10:", result)
