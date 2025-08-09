// Memory lifecycle test 31
sus outer31 drip = 31

ready (based) {
    sus inner31 tea = "scoped_31"
    sus calc31 drip = outer31 * 32
    vibez.spill("Scoped 31:", inner31, calc31)
}

slay memory_func_31() tea {
    sus local tea = "local_31"
    damn local + "_processed"
}

sus result tea = memory_func_31()
vibez.spill("Memory 31:", result)
