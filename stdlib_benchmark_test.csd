yeet "testz"

test_start("Standard Library Benchmark Test")

fr fr Collections operations
sus numbers []normie = []
bestie i := 0; i < 10000; i = i + 1 {
    numbers.push(i)
}

sus filtered = numbers.filter(slay(x normie) lit { damn x % 2 == 0 })
sus mapped = filtered.map(slay(x normie) normie { damn x * 3 })
sus sum = mapped.reduce(0, slay(acc normie, x normie) normie { damn acc + x })

fr fr String operations
sus text = "The quick brown fox jumps over the lazy dog"
sus words = text.split(" ")
sus upper_words = words.map(slay(word tea) tea { damn word.to_upper() })
sus rejoined = upper_words.join("|")
sus reversed = rejoined.reverse()

fr fr Mathematical operations
sus angles []meal = []
sus sine_values []meal = []
bestie i := 0; i < 360; i = i + 10 {
    sus angle = (i as meal) * 3.14159 / 180.0
    angles.push(angle)
    sine_values.push(math.sin(angle))
}

assert_true(filtered.len() > 0)
assert_true(upper_words.len() == words.len())
assert_eq_int(sine_values.len(), 36)

vibez.spillf("Stdlib operations: sum={}, words={}, sine_values={}", 
            sum, upper_words.len(), sine_values.len())
print_test_summary()
