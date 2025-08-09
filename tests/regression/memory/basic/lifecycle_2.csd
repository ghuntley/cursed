// Memory lifecycle test 2
sus outer2 drip = 2

ready (based) {
    sus inner2 tea = "scoped_2"
    sus calc2 drip = outer2 * 3
    vibez.spill("Scoped 2:", inner2, calc2)
}

slay memory_func_2() tea {
    sus local tea = "local_2"
    damn local + "_processed"
}

sus result tea = memory_func_2()
vibez.spill("Memory 2:", result)
