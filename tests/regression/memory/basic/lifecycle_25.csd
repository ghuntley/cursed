// Memory lifecycle test 25
sus outer25 drip = 25

ready (based) {
    sus inner25 tea = "scoped_25"
    sus calc25 drip = outer25 * 26
    vibez.spill("Scoped 25:", inner25, calc25)
}

slay memory_func_25() tea {
    sus local tea = "local_25"
    damn local + "_processed"
}

sus result tea = memory_func_25()
vibez.spill("Memory 25:", result)
