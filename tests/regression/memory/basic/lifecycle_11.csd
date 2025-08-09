// Memory lifecycle test 11
sus outer11 drip = 11

ready (based) {
    sus inner11 tea = "scoped_11"
    sus calc11 drip = outer11 * 12
    vibez.spill("Scoped 11:", inner11, calc11)
}

slay memory_func_11() tea {
    sus local tea = "local_11"
    damn local + "_processed"
}

sus result tea = memory_func_11()
vibez.spill("Memory 11:", result)
