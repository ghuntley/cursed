// Memory lifecycle test 24
sus outer24 drip = 24

ready (based) {
    sus inner24 tea = "scoped_24"
    sus calc24 drip = outer24 * 25
    vibez.spill("Scoped 24:", inner24, calc24)
}

slay memory_func_24() tea {
    sus local tea = "local_24"
    damn local + "_processed"
}

sus result tea = memory_func_24()
vibez.spill("Memory 24:", result)
