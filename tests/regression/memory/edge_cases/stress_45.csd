// Memory stress test 45
slay create_strings_45(count drip) []tea {
    sus strings []tea = []
    sus j drip = 0
    bestie (j < count) {
        sus str tea = "string_" + to_str_drip(j) + "_test_45"
        strings = append_tea(strings, str)
        j = j + 1
    }
    damn strings
}

sus result []tea = create_strings_45(90)
vibez.spill("Memory stress 45:", len(result))
