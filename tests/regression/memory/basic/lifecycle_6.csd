// Memory lifecycle test 6
sus outer6 drip = 6

ready (based) {
    sus inner6 tea = "scoped_6"
    sus calc6 drip = outer6 * 7
    vibez.spill("Scoped 6:", inner6, calc6)
}

slay memory_func_6() tea {
    sus local tea = "local_6"
    damn local + "_processed"
}

sus result tea = memory_func_6()
vibez.spill("Memory 6:", result)
