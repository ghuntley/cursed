// Memory lifecycle test 14
sus outer14 drip = 14

ready (based) {
    sus inner14 tea = "scoped_14"
    sus calc14 drip = outer14 * 15
    vibez.spill("Scoped 14:", inner14, calc14)
}

slay memory_func_14() tea {
    sus local tea = "local_14"
    damn local + "_processed"
}

sus result tea = memory_func_14()
vibez.spill("Memory 14:", result)
