// Memory lifecycle test 34
sus outer34 drip = 34

ready (based) {
    sus inner34 tea = "scoped_34"
    sus calc34 drip = outer34 * 35
    vibez.spill("Scoped 34:", inner34, calc34)
}

slay memory_func_34() tea {
    sus local tea = "local_34"
    damn local + "_processed"
}

sus result tea = memory_func_34()
vibez.spill("Memory 34:", result)
