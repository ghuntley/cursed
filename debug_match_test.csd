vibez.spill("Before match")

sus test_value normie = 42
vibez.spill("Test value is 42")

sus result := match test_value {
    42 -> "answer"
}

vibez.spill("After match")
vibez.spill(result)
