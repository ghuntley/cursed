// Memory lifecycle test 21
sus outer21 drip = 21

ready (based) {
    sus inner21 tea = "scoped_21"
    sus calc21 drip = outer21 * 22
    vibez.spill("Scoped 21:", inner21, calc21)
}

slay memory_func_21() tea {
    sus local tea = "local_21"
    damn local + "_processed"
}

sus result tea = memory_func_21()
vibez.spill("Memory 21:", result)
