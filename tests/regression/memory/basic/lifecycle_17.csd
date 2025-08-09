// Memory lifecycle test 17
sus outer17 drip = 17

ready (based) {
    sus inner17 tea = "scoped_17"
    sus calc17 drip = outer17 * 18
    vibez.spill("Scoped 17:", inner17, calc17)
}

slay memory_func_17() tea {
    sus local tea = "local_17"
    damn local + "_processed"
}

sus result tea = memory_func_17()
vibez.spill("Memory 17:", result)
