// Memory lifecycle test 15
sus outer15 drip = 15

ready (based) {
    sus inner15 tea = "scoped_15"
    sus calc15 drip = outer15 * 16
    vibez.spill("Scoped 15:", inner15, calc15)
}

slay memory_func_15() tea {
    sus local tea = "local_15"
    damn local + "_processed"
}

sus result tea = memory_func_15()
vibez.spill("Memory 15:", result)
