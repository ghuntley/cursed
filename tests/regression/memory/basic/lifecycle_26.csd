// Memory lifecycle test 26
sus outer26 drip = 26

ready (based) {
    sus inner26 tea = "scoped_26"
    sus calc26 drip = outer26 * 27
    vibez.spill("Scoped 26:", inner26, calc26)
}

slay memory_func_26() tea {
    sus local tea = "local_26"
    damn local + "_processed"
}

sus result tea = memory_func_26()
vibez.spill("Memory 26:", result)
