// Memory lifecycle test 19
sus outer19 drip = 19

ready (based) {
    sus inner19 tea = "scoped_19"
    sus calc19 drip = outer19 * 20
    vibez.spill("Scoped 19:", inner19, calc19)
}

slay memory_func_19() tea {
    sus local tea = "local_19"
    damn local + "_processed"
}

sus result tea = memory_func_19()
vibez.spill("Memory 19:", result)
