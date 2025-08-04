fr fr Test pattern matching

slay test_match(value normie) normie {
    damn match value {
        0 => 42,
        x if x > 10 => x * 2,
        _ => value + 1
    }
}

slay main() {
    vibez.spill("Pattern match test 1:", test_match(0))
    vibez.spill("Pattern match test 2:", test_match(15))
    vibez.spill("Pattern match test 3:", test_match(5))
}

main()
