// Variable lifecycle and memory management
sus outer drip = 42

ready (based) {
    sus inner tea = "scoped variable"
    sus calculated drip = outer * 2
    vibez.spill("Inner scope:", inner, calculated)
}

sus array []drip = [1, 2, 3, 4, 5]
sus i drip = 0
bestie (i < len(array)) {
    sus temp drip = array[i] * 2
    vibez.spill("Temp:", temp)
    i = i + 1
}

slay memory_test() {
    sus local tea = "local variable"
    sus result drip = len_str(local)
    damn result
}

sus func_result drip = memory_test()
vibez.spill("Function result:", func_result)
