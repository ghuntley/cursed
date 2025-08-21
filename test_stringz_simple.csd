fr fr Simple test of StringZ core module

yeet "stringz"

fr fr Test basic functionality
sus parts []tea = split("a,b,c", ",")
ready (len(parts) == 3) {
    vibez.spill("✅ Split test passed:", len(parts))
} otherwise {
    vibez.spill("❌ Split test failed")
}

sus joined tea = join(["hello", "world"], " ")
ready (joined == "hello world") {
    vibez.spill("✅ Join test passed:", joined)
} otherwise {
    vibez.spill("❌ Join test failed")
}

sus replaced tea = replace("hello world", "hello", "hi")
ready (replaced == "hi world") {
    vibez.spill("✅ Replace test passed:", replaced)
} otherwise {
    vibez.spill("❌ Replace test failed")
}

sus num drip = parse_int("42")
ready (num == 42) {
    vibez.spill("✅ Parse int test passed:", num)
} otherwise {
    vibez.spill("❌ Parse int test failed")
}

sus valid lit = is_numeric("123")
ready (valid == based) {
    vibez.spill("✅ Is numeric test passed")
} otherwise {
    vibez.spill("❌ Is numeric test failed")
}

vibez.spill("StringZ core module basic tests completed!")
