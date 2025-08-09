// Memory lifecycle test 33
sus outer33 drip = 33

ready (based) {
    sus inner33 tea = "scoped_33"
    sus calc33 drip = outer33 * 34
    vibez.spill("Scoped 33:", inner33, calc33)
}

slay memory_func_33() tea {
    sus local tea = "local_33"
    damn local + "_processed"
}

sus result tea = memory_func_33()
vibez.spill("Memory 33:", result)
