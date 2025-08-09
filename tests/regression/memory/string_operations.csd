// String operations that could cause memory leaks
yeet "stringz"

sus base tea = "hello"
sus extended tea = base + " world"
sus upper tea = to_upper(extended)
sus trimmed tea = trim_str("  " + upper + "  ")

// String operations in loops
sus i drip = 0
sus accumulated tea = ""
bestie (i < 5) {
    accumulated = accumulated + " " + to_str_drip(i)
    i = i + 1
}

vibez.spill("Final string:", trimmed)
vibez.spill("Accumulated:", accumulated)

// Function with string returns
slay build_string(count drip) tea {
    sus result tea = ""
    sus j drip = 0
    bestie (j < count) {
        result = result + "x"
        j = j + 1
    }
    damn result
}

sus built tea = build_string(10)
vibez.spill("Built string:", built)
