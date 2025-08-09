// Memory lifecycle test 4
sus outer4 drip = 4

ready (based) {
    sus inner4 tea = "scoped_4"
    sus calc4 drip = outer4 * 5
    vibez.spill("Scoped 4:", inner4, calc4)
}

slay memory_func_4() tea {
    sus local tea = "local_4"
    damn local + "_processed"
}

sus result tea = memory_func_4()
vibez.spill("Memory 4:", result)
