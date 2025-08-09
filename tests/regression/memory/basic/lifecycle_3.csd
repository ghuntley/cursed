// Memory lifecycle test 3
sus outer3 drip = 3

ready (based) {
    sus inner3 tea = "scoped_3"
    sus calc3 drip = outer3 * 4
    vibez.spill("Scoped 3:", inner3, calc3)
}

slay memory_func_3() tea {
    sus local tea = "local_3"
    damn local + "_processed"
}

sus result tea = memory_func_3()
vibez.spill("Memory 3:", result)
