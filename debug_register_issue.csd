// Minimal test to debug register numbering issue
slay test() {
    sus numbers []normie = [1, 2, 3]
    sus first normie = numbers[0]
    vibez.spill(first)
}
