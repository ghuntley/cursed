// Memory stress test 41
slay create_strings_41(count drip) []tea {
    sus strings []tea = []
    sus j drip = 0
    bestie (j < count) {
        sus str tea = "string_" + to_str_drip(j) + "_test_41"
        strings = append_tea(strings, str)
        j = j + 1
    }
    damn strings
}

sus result []tea = create_strings_41(82)
vibez.spill("Memory stress 41:", len(result))
