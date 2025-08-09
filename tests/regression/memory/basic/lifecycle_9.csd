// Memory lifecycle test 9
sus outer9 drip = 9

ready (based) {
    sus inner9 tea = "scoped_9"
    sus calc9 drip = outer9 * 10
    vibez.spill("Scoped 9:", inner9, calc9)
}

slay memory_func_9() tea {
    sus local tea = "local_9"
    damn local + "_processed"
}

sus result tea = memory_func_9()
vibez.spill("Memory 9:", result)
