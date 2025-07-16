sus test_value normie = 42

sus result := match test_value {
    1 -> "one"
    2 -> "two"
    42 -> "answer"
    _ -> "other"
}

vibez.spill(result)

sus bool_test lit = based
sus bool_result := match bool_test {
    based -> "true case"
    cap -> "false case"
}

vibez.spill(bool_result)

sus range_test normie = 15
sus range_result := match range_test {
    1..10 -> "small"
    11..20 -> "medium"
    21..30 -> "large"
    _ -> "unknown"
}

vibez.spill(range_result)
