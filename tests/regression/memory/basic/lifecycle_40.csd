// Memory lifecycle test 40
sus outer40 drip = 40

ready (based) {
    sus inner40 tea = "scoped_40"
    sus calc40 drip = outer40 * 41
    vibez.spill("Scoped 40:", inner40, calc40)
}

slay memory_func_40() tea {
    sus local tea = "local_40"
    damn local + "_processed"
}

sus result tea = memory_func_40()
vibez.spill("Memory 40:", result)
